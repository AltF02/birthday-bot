use serenity::{
    framework::standard::{
        Args, CheckResult, CommandOptions,
        macros::check,
    },
    model::{
        channel::Message,
    },
};

use serenity::prelude::*;

#[check]
#[name = "Owner"]
async fn owner_check(_: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    (msg.author.id == 357918459058978816).into()
}