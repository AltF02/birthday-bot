use crate::bot::utils::{
    calculate_age, check_birthday_noted, check_msg, comp_reply, confirm, parse_member, reply,
};
use crate::bot::{ConnectionPool, ShardManagerContainer};
use crate::config::Config;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use std::time::{Duration, Instant};

use log::warn;

use chrono::prelude::*;
use serenity::client::bridge::gateway::ShardId;
use serenity::model::guild::Member;
use serenity::model::user::User;
use serenity::utils::Colour;

#[group()]
#[commands(ping, prefix, set, birthday, avatar)]
pub struct General;

#[command]
#[aliases("pong", "latency")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let shard_manager = data.get::<ShardManagerContainer>().unwrap();

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = runners.get(&ShardId(ctx.shard_id)).unwrap();

    let shard_latency = match runner.latency {
        Some(ms) => format!("{:.2}ms", ms.as_micros() as f32 / 1000.0),
        _ => String::new(),
    };

    let now = Instant::now();
    let mut message = match comp_reply(&ctx, &msg, &"Calculating...".to_string()).await {
        Ok(m) => m,
        Err(why) => {
            warn!("Unable to send message: {}", why);
            return Ok(());
        }
    };
    let rest_latency = format!("{:.2}ms", now.elapsed().as_micros() as f32 / 1000.0);

    message
        .edit(ctx, |m| {
            m.content("");
            m.embed(|e| {
                e.title("Pong! 🏓");
                e.color(0xffa500);
                e.description(format!("*WS:* {}\n*REST:* {}", shard_latency, rest_latency))
            })
        })
        .await
        .unwrap();

    Ok(())
}

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    if let Err(why) = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|embed| {
                embed.title("Prefix");
                embed.description(format!("My prefix is: `{}`", &config.prefix));
                embed.color(0xffa500)
            });
            m
        })
        .await
    {
        warn!(
            "Failed to send message in #{} because\n{:?}",
            msg.channel_id, why
        );
    };

    Ok(())
}

#[command]
async fn set(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    if let Some(_date) = check_birthday_noted(msg.author.id.0 as i64, &pool).await {
        msg.reply(&ctx, "You already have a birthday set").await.unwrap();
    } else {
        reply(
            &ctx,
            &msg,
            &format!(
                "Hey there **{}**, please enter your birthday. `[ DD/MM/YYYY ]`",
                msg.author.name
            ),
        )
        .await;
        let date: NaiveDate;

        if let Some(answer) = &msg
            .author
            .await_reply(&ctx)
            .timeout(Duration::from_secs(20))
            .await
        {
            date = NaiveDate::parse_from_str(&answer.content.as_str(), "%d/%m/%Y").unwrap();
        } else {
            reply(&ctx, &msg, &String::from("Bruh you didn't reply stupid")).await;
            return Ok(());
        }

        let today: NaiveDate = Utc::today().naive_utc();

        if date > today {
            reply(
                &ctx,
                &msg,
                &"Nope you can't exist in the future smh".to_string(),
            )
            .await;
            return Ok(());
        }

        let age = calculate_age(date);

        match age {
            0..=12 => {
                reply(&ctx, &msg, &format!("You have to be at least **13** to use Discord **{}**, are you saying that you're underage!? 🤔", msg.author.name)).await;
            }
            13..=116 => {
                if confirm(
                    &ctx,
                    &msg,
                    &"Is this correct?".to_string(),
                    &format!(
                        "Is your age **{}** and your birthday is on **{}**?",
                        age.to_string(),
                        date.format("%d %B").to_string()
                    ),
                )
                .await
                {
                    /*                    if let Err(why) = db
                        .execute(
                            "INSERT INTO birthdaybot.birthdays VALUES ($1, $2)",
                            &[&(msg.author.id.0 as i64), &date],
                        )
                        .await
                    {
                        warn!("Unable to insert into database {:?}", why)
                    };*/
                    let user_id = msg.author.id.0 as i64;
                    sqlx::query!(
                        "INSERT INTO birthdaybot.birthdays VALUES ($1, $2)",
                        user_id,
                        &date
                    )
                    .execute(pool)
                    .await?;

                    reply(&ctx, &msg, &"Confirmed!".to_string()).await;
                }
            }
            _ => {
                reply(&ctx, &msg, &format!("The oldest person in the world is **116**, doubtful that you're that old **{}**...", msg.author.name)).await;
            }
        }
    }

    // reply(&ctx, &msg, &format!("{}", confirm(&ctx, &msg, &"Title".to_string(), &"Description".to_string()).await)).await;
    Ok(())
}

#[command]
#[aliases("birth", "b")]
async fn birthday(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user: User = match args.single_quoted::<String>() {
        Ok(arg) => match parse_member(ctx, msg, arg).await {
            Some(m) => m.user,
            None => {
                reply(ctx, msg, &"Unable to locate user".to_string()).await;
                return Ok(());
            }
        },
        Err(_e) => msg.author.to_owned(),
    };

    let data = ctx.data.read().await;
    let pool = data.get::<ConnectionPool>().unwrap();

    if user.id == ctx.cache.current_user_id().await {
        reply(
            ctx,
            msg,
            &"Sadly robots don't have birthdays :(".to_string(),
        )
        .await;
        return Ok(());
    }

    let birthday_noted = check_birthday_noted(user.id.0 as i64, &pool).await;
    if let None = birthday_noted {
        reply(
            ctx,
            msg,
            &format!(
                "**{}** has not saved his/her/their birthday yet :(",
                user.name
            ),
        )
        .await;
        return Ok(());
    }
    let birthday: NaiveDate = birthday_noted.unwrap();
    let age: i32 = calculate_age(birthday);

    check_msg(
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|embed| {
                    embed.title(format!("{}'s birthday", user.name));
                    embed.thumbnail(user.face());
                    embed.description(format!(
                        "🍰 Birthday: **{}**\n📅 Age: **{}**",
                        birthday.format("%d %B").to_string(),
                        age
                    ));
                    embed.colour(0xffa500)
                });
                m
            })
            .await,
    );
    Ok(())
}

#[command]
#[aliases("av", "pfp")]
#[only_in("guilds")]
async fn avatar(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let member: Member = match args.single_quoted::<String>() {
        Ok(arg) => match parse_member(ctx, msg, arg).await {
            Some(m) => m,
            None => {
                reply(ctx, msg, &"Unable to locate user".to_string()).await;
                return Ok(());
            }
        },
        Err(_e) => {
            let guild = msg.guild_id.unwrap();
            guild.member(ctx, msg.author.id).await.unwrap()
        }
    };
    let colour = member.colour(ctx).await.unwrap_or(Colour::new(0xffa500));
    check_msg(
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|embed| {
                    embed.title(format!("{} looking kinda secksy", member.display_name()));
                    embed.image(member.user.face());
                    embed.colour(colour)
                });
                m
            })
            .await,
    );
    Ok(())
}
