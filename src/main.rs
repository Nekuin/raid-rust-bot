use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;
use std::sync::Arc;
use tokio::main;
use tokio::sync::RwLock;

mod commands;
mod events;
use crate::events::interactions::Handler;
mod model;
use crate::model::raid::RaidList;

#[group]
#[commands(ping)]
struct General;

#[main(flavor = "current_thread")]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        // init RaidList into context
        let mut data = client.data.write().await;
        data.insert::<RaidList>(Arc::new(RwLock::new(RaidList::new())));
    }

    if let Err(why) = client.start().await {
        println!("An error occured while running the client: {:?}", why)
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Ping command!");
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
