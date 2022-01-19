#![feature(destructuring_assignment)]

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::env;

#[group]
#[commands(ping, help)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let help = "AMOGUS";

    let dm = msg.author.direct_message(&ctx, |m| {
        m.content(&help)
    }).await;

    match dm {
        Ok(_) => {
            _ = msg.react(&ctx, '👌').await;
        }
        Err(err) => {
            msg.reply(&ctx, "You're not worty").await;
            eprintln!("Well shit");
        }
    }
    Ok(())
}
