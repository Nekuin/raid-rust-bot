use serenity::async_trait;
use serenity::model::channel::Reaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use std::env;

use crate::commands::raid::handle_raid_command;

pub struct Handler;

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
        println!("Reaction add: {:?}", reaction);
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        println!("Reaction remove: {:?}", reaction);
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
                        .description("Pakko ottaa")
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
