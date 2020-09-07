use serenity::{
    prelude::*,
    framework::standard::{
        CommandResult,
        macros::{
            command, group
        }

    },
    model::{
        channel::Message
    }
};

use crate::bot::utils::reply;


#[group()]
#[prefixes("owner")]
#[default_command(test)]
#[owners_only]
#[commands(yeet)]
pub struct Owner;

#[command]
#[aliases("gay", "stupid")]
async fn yeet(ctx: &Context, msg: &Message) -> CommandResult {
    reply(&ctx, &msg, &"Hey if you see this the check worked!".to_string()).await;
    Ok(())
}

#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    reply(&ctx, &msg, &"Gay".to_string()).await;
    Ok(())
}