use serenity::{
    client::Context,
    model::{
        channel::ReactionType,
        id::EmojiId,
        interactions::application_command::{
            ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
        },
    },
};

use crate::model::raid::{Raid, RaidList};

/**
 * Handles /raid command which is used to create a new raid.
 */
pub async fn handle_raid_command(command: &ApplicationCommandInteraction, ctx: &Context) -> String {
    // get option values from command data
    let opts = command
        .data
        .options
        // 0, 1, 2 inclusive
        .get(0..=2)
        .expect("Expected to find 3 arguments");

    // get the string values of the options
    let mut resolved_options: Vec<String> = vec![];
    for opt in opts {
        if let ApplicationCommandInteractionDataOptionValue::String(string) =
            opt.resolved.as_ref().expect("Expected string object")
        {
            resolved_options.push(string.to_string())
        }
    }
    // log option values
    println!("{:#?} options?", resolved_options);

    // reference the option values from resolved_options
    let time = resolved_options.get(0).unwrap();
    let boss = resolved_options.get(1).unwrap();
    let location = resolved_options.get(2).unwrap();
    // create a new raid instance
    let new_raid = Raid::new(time, location, boss);
    println!("{:?} new raid", new_raid);

    // obtain a lock to our RaidList from ctx.data
    let raids_lock = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<RaidList>()
            .expect("Expected RaidList in TypeMap")
            .clone()
    };
    // use a scope so raid lock is released automatically
    // after we exit the scope
    {
        // open lock to write mode
        let mut raids = raids_lock.write().await;
        // add raid to raid list
        raids.add_raid(location, &new_raid);
        // log all raids
        println!("{:?} raids", raids);
    }

    // send an embed message containing the raid information,
    // and add sign up reactions to the message
    let raid_message = command
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| new_raid.as_embed(e)).reactions(vec![
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(503269083953758265),
                    name: Some("1_".to_string()),
                },
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(503269083731460107),
                    name: Some("2_".to_string()),
                },
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(503269084075393024),
                    name: Some("3_".to_string()),
                },
            ])
        })
        .await;

    // log error
    if let Err(why) = &raid_message {
        println!("Failed to send msg: {:?}", why)
    } else if let Ok(rm) = raid_message {
        // obtain a lock to our RaidList again from ctx.data
        let raids_lock = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<RaidList>()
                .expect("Expected RaidList in TypeMap")
                .clone()
        };
        // use a scope so raid lock is released automatically
        // after we exit the scope
        {
            // open lock to write mode
            let mut raids = raids_lock.write().await;
            raids.add_raid_by_message(&rm.id, &location)
        }
    }

    // add a message to the slash command for the calling user
    "Hienosti meni".to_string()
}
