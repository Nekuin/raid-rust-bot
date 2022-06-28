use serenity::async_trait;
use serenity::model::channel::Reaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use std::env;

use crate::commands::raid::handle_raid_command;
use crate::model::raid::{RaidList, Raider};

pub struct Handler;

/**
 * Helper function to check if reaction adder/remover is a bot
 */
fn is_bot(reaction: &Reaction) -> Option<bool> {
    reaction
        .member
        .as_ref()
        .and_then(|m| m.user.as_ref())
        .and_then(|u| Some(u.bot))
}

fn emoji_name_from_string(string: String) -> String {
    let split = string.split(":");
    let parts = split.collect::<Vec<&str>>();
    match parts.get(0) {
        None => "".to_string(),
        Some(string) => String::from(*string),
    }
}

fn count_from_emoji_name(emoji_name: String) -> u8 {
    let mut count: u8 = 1;
    // in a "production version" this would
    // probably be a map for each server using this bot.
    if emoji_name == "1_" {
        count = 1;
    } else if emoji_name == "2_" {
        count = 2;
    } else if emoji_name == "3_" {
        count = 3;
    }
    count
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = &interaction {
            println!("Recieved command interaction: {:#?}", command);
            let content = match command.data.name.as_str() {
                "ping" => "... Olen".to_string(),
                "raid" => handle_raid_command(&command, &ctx).await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // check that the reaction adder is not a bot
        // default being bot to true, we should always have information about the user.
        if is_bot(&reaction).unwrap_or(true) == false {
            println!("Reaction add: {:?}", reaction);
            // obtain a lock to our RaidList from ctx.data
            let raids_lock = {
                let data_read = ctx.data.read().await;
                data_read
                    .get::<RaidList>()
                    .expect("Expected RaidList in TypeMap")
                    .clone()
            };
            // use a scope so write lock is released automatically
            // after we exit the scope
            {
                // open lock to write mode
                let mut raid_list = raids_lock.write().await;
                // add raid to raid list
                match raid_list.msg_to_location.get(&reaction.message_id) {
                    None => {
                        println!(
                            "Didn't find a location based on message id {:?}",
                            reaction.message_id
                        );
                    }
                    Some(location) => match raid_list.raids.get(location) {
                        None => {
                            println!("Didn't find a raid based on location {:?}", location);
                        }
                        Some(raid) => {
                            // create a raider from reaction member
                            let raider = Raider::new(reaction.member);
                            // create a mutable reference to raid
                            let r = &mut raid.clone();

                            let count: u8 = count_from_emoji_name(emoji_name_from_string(
                                reaction.emoji.as_data(),
                            ));

                            // add raider to raid
                            r.add_raider(raider, count);

                            println!("Added new raider! {:?}", r);

                            // edit raid message
                            let edit_message = reaction
                                .channel_id
                                .edit_message(&ctx.http, reaction.message_id, |m| {
                                    m.embed(|e| r.clone().as_embed(e))
                                })
                                .await;
                            if let Err(why) = &edit_message {
                                // log if editing fails
                                println!("Failed to edit message: {:?}", why)
                            } else if let Ok(_msg) = edit_message {
                                // update raid_list in context
                                raid_list.update_raid(&r.location, &r);
                                println!("Edited message!");
                            }
                        }
                    },
                }
            }
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        // check that the reaction adder is not a bot
        // default being bot to false, seems like reaction remove calls
        // only really have the user_id in the data.
        if is_bot(&reaction).unwrap_or(false) == false {
            println!("Reaction remove: {:?}", reaction);

            // obtain a lock to our RaidList from ctx.data
            let raids_lock = {
                let data_read = ctx.data.read().await;
                data_read
                    .get::<RaidList>()
                    .expect("Expected RaidList in TypeMap")
                    .clone()
            };
            // use a scope so write lock is released automatically
            // after we exit the scope
            {
                // open lock to write mode
                let mut raid_list = raids_lock.write().await;
                // add raid to raid list
                match raid_list.msg_to_location.get(&reaction.message_id) {
                    None => {
                        println!(
                            "Didn't find a location based on message id {:?}",
                            reaction.message_id
                        );
                    }
                    Some(location) => match raid_list.raids.get(location) {
                        None => {
                            println!("Didn't find a raid based on location {:?}", location);
                        }
                        Some(raid) => {
                            // create a mutable reference to raid
                            let r = &mut raid.clone();
                            match reaction.user_id {
                                None => {
                                    println!("No user id in reaction {:?}", reaction);
                                }
                                Some(user_id) => {
                                    let count: u8 = count_from_emoji_name(emoji_name_from_string(
                                        reaction.emoji.as_data(),
                                    ));
                                    // remove raider from raider
                                    r.remove_raider(*user_id.as_u64(), count);

                                    println!("Added new raider! {:?}", r);

                                    // edit raid message
                                    let edit_message = reaction
                                        .channel_id
                                        .edit_message(&ctx.http, reaction.message_id, |m| {
                                            m.embed(|e| r.clone().as_embed(e))
                                        })
                                        .await;
                                    if let Err(why) = &edit_message {
                                        // log if editing fails
                                        println!("Failed to edit message: {:?}", why)
                                    } else if let Ok(_msg) = edit_message {
                                        // update raid_list in context
                                        raid_list.update_raid(&r.location, &r);
                                        println!("Edited message!");
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("Oletko valmis?")
                })
                .create_application_command(|command| {
                    command
                        .name("raid")
                        .description("Luo uuden raidin")
                        .create_option(|option| {
                            option
                                .name("aika")
                                .description("Raidin kellonaika")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("pomo")
                                .description("Raidin pomo")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("paikka")
                                .description("Raidin paikka")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
        })
        .await;
        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }
}
