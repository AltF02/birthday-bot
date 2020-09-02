use serenity::prelude::Context;
use serenity::model::channel::Message;

pub(crate) async fn reply(ctx: &Context, msg: &Message, content: &String) {
    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        println!("Failed to send message in #{} because\n{:?}",
                 msg.channel_id, why
        );
    }
}

/*
pub(crate) async fn reply_embed<T>(ctx: &Context, msg: &Message, embed: T) {
    if let Err(why) = msg.channel_id.send_message(&ctx.http, &embed).await {
        println!("Failed to send message in #{} because\n{:?}",
                 msg.channel_id, why
        );
    }
}
*/