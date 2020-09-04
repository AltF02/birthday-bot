use serenity::prelude::Context;
use serenity::model::channel::Message;
use chrono::{NaiveDate, Utc, Datelike};
use serenity::model::channel::ReactionType::Unicode;
use std::time::Duration;

pub(crate) async fn reply(ctx: &Context, msg: &Message, content: &String) {
    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        println!("Failed to send message in #{} because\n{:?}",
                 msg.channel_id, why
        );
    }
}

pub(crate) fn calculate_age(born: NaiveDate) -> i32 {
    let today: NaiveDate = Utc::today().naive_utc();
    let mut age: i32 = today.year() - born.year();
    let month: i32 = (today.month() as i32) - (born.month() as i32);
    if month < 0 || (month == 0 && today.day() < born.day()) {
        age -= 1
    }
    age
}

pub(crate) async fn confirm(ctx: &Context, msg: &Message, title: &String, description: &String) -> bool {
    let conf_msg =  msg.channel_id.send_message(&ctx.http,  |m| {
        m.embed(|embed| {
            embed.title(title);
            embed.description(description);
            embed.color(0xffa500)
        });

        m.reactions(vec![Unicode("✅".to_string()), Unicode("❌".to_string())]);
        m

    }).await;

    return match conf_msg {
        Ok(conf_msg) => {
            if let Some(reaction) = &conf_msg.await_reaction(&ctx).timeout(Duration::from_secs(10)).author_id(msg.author.id).await {
                let emoji = &reaction.as_inner_ref().emoji;

                match emoji.as_data().as_str() {
                    "✅" => { true }
                    "❌" => {
                        reply(&ctx, &msg, &"Please restart the process".to_string()).await;
                        false
                    }
                    _ => {
                        reply(&ctx, &msg, &"Bruh don't add more reactions, start again smh. This is why the human race is a mistake".to_string()).await;
                        false
                    }
                }
            } else {
                reply(&ctx, &msg, &"What the heck you didn't react".to_string()).await;
                false
            }
        }
        Err(why) => {
            println!("Failed to send message in #{} because\n{:?}",
                     msg.channel_id, why
            );
            false
        }
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