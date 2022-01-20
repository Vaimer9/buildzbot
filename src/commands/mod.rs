#![feature(destructuring_assignment)]

use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;

const DISCORD_VAR: &str = "DISCORD_VAR";

use std::env;

enum DataCall {
    Get,
    Set(String)
}

fn access_ip(procedure: DataCall) -> Result<String, ()> {
    match procedure {
        DataCall::Get => {
            Ok(env::var(DISCORD_VAR).expect("Set the var dude"))
        }
        DataCall::Set(x) => {
            env::set_var(DISCORD_VAR, x);
            Err(())
        }
    }
}

#[command]
pub async fn ip(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, &access_ip(DataCall::Get).unwrap()).await?;

    Ok(())
}

#[command]
pub async fn changeip(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let temp = args.single::<String>()?;
    access_ip(DataCall::Set(temp));

    _ = msg.react(&ctx, '👌').await?;

    Ok(())
}
