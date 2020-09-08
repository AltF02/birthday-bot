use chrono::{NaiveDate, Utc, Datelike};
use std::time::Duration;
use serenity::{
    prelude::*,
    model::prelude::ReactionType::Unicode,
    model::prelude::*,
    Error,
    Result as SerenityResult
};

use log::{
    warn
};

use tokio_postgres::Client as DBClient;
use regex::Regex;

pub(crate) async fn reply(ctx: &Context, msg: &Message, content: &String) {
    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        warn!(
            "Failed to send message in #{} because\n{:?}",
            msg.channel_id, why
        );
    }
}

pub(crate) async fn comp_reply(ctx: &Context, msg: &Message, content: &String) -> Result<Message, Error> {
    return msg.channel_id.say(&ctx.http, &content).await
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
            warn!("Failed to send message in #{} because\n{:?}",
                     msg.channel_id, why
            );
            false
        }
    }
}

pub(crate) fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        warn!("Error sending message: {:?}", why);
    }
}

pub(crate) async fn check_birthday_noted(msg: &Message, db: &DBClient) -> bool {
    let user = db.query("SELECT date FROM birthdaybot.birthdays WHERE user_id = $1", &[&(msg.author.id.0 as i64)])
        .await
        .unwrap();

    !user.is_empty()
}

pub(crate) async fn parse_member(ctx: &Context, msg: &Message, member_name: String) -> Option<Member> {
    let member: Member;
    if let Ok(id) = member_name.parse::<u64>() {
        member = match msg.guild_id.unwrap().member(ctx, id).await {
            Ok(m) => m,
            Err(_e) => {
                return None
            }
        };
        Some(member.to_owned())
    } else if member_name.starts_with("<@") && member_name.ends_with(">") {
        let re = Regex::new("[<@!>]").unwrap();
        let member_id = re.replace_all(&member_name, "").into_owned();

        member = match msg.guild_id.unwrap().member(ctx, UserId(member_id.parse::<u64>().unwrap())).await {
            Ok(m) => m,
            Err(_e) => {
                return None
            }
        };

        Some(member.to_owned())
    } else {
        None
    }
}