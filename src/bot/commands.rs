use serenity::{
    prelude::*,
    framework::standard::{
        Args,
        CommandResult,
        macros::{
            command, group
        }

    },
    model::{
        channel::Message
    }
};
use crate::bot::utils::{reply, calculate_age, confirm};
use crate::config::Config;
use crate::bot::DataBase;
use std::{
    time::Duration
};
use chrono::prelude::*;

#[group()]
#[commands(ping, prefix, db_test, set)]
pub struct Commands;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let test = "fortnite";
    println!("{}", test);

    reply(&ctx, &msg, &String::from("Pong!")).await;
    Ok(())
}

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    if let Err(why) = msg.channel_id.send_message(&ctx.http,  |m| {
        m.embed(|embed| {
            embed.title("Prefix");
            embed.description(format!("My prefix is: `{}`", &config.prefix));
            embed.color(0xffa500)
        });
        m

    }).await {
        println!("Failed to send message in #{} because\n{:?}",
                 msg.channel_id, why
        );
    };

    Ok(())
}

#[command]
async fn db_test(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let rows = db.query("SELECT yeet FROM birthdaybot.test", &[]).await.unwrap();

    reply(&ctx, &msg, &rows[0].get(0)).await;

    Ok(())
}

#[command]
async fn wait_for(ctx: &Context, msg: &Message) -> CommandResult {
    reply(&ctx, &msg, &"Please enter something in 10 seconds".to_string()).await;
    if let Some(answer) = &msg.author.await_reply(&ctx).timeout(Duration::from_secs(10)).await {
        reply(&ctx, &msg, &answer.content).await;
    } else {
        reply(&ctx, &msg, &String::from("Bruh you didn't reply stupid")).await;
    }
    Ok(())
}

#[command]
async fn set(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let user = db.query("SELECT date FROM birthdaybot.birthdays WHERE user_id = $1", &[&(msg.author.id.0 as i64)])
        .await
        .unwrap();

    if !user.is_empty() {
        reply(&ctx, &msg, &String::from("You already have a birthday set")).await;

    } else {
        reply(&ctx, &msg, &format!("Hey there **{}**, please enter your birthday. `[ DD/MM/YYYY ]`", msg.author.name)).await;
        let date: NaiveDate;

        if let Some(answer) = &msg.author.await_reply(&ctx).timeout(Duration::from_secs(20)).await {
            date = NaiveDate::parse_from_str(&answer.content.as_str(), "%d/%m/%Y").unwrap();

        } else {
            reply(&ctx, &msg, &String::from("Bruh you didn't reply stupid")).await;
            return Ok(())
        }

        let today: NaiveDate = Utc::today().naive_utc();

        if date > today {
            reply(&ctx, &msg, &"Nope you can't exist in the future smh".to_string()).await;
            return Ok(())
        }

        let age = calculate_age(date);

        match age {
            0 ..= 12 => {
                reply(&ctx, &msg, &format!("You have to be at least **13** to use Discord **{}**, are you saying that you're underage!? 🤔", msg.author.name)).await;
            }
            13..= 116 => {
                let conf = confirm(&ctx, &msg, &"Is this correct?".to_string(), &format!("Is your age **{}** and your birthday is on **{}**?", age.to_string(), date.to_string())).await;

                match conf {
                    true => {
                        if let Err(why) = db.execute("INSERT INTO birthdaybot.birthdays VALUES ($1, $2)", &[&(msg.author.id.0 as i64), &date]).await {
                            println!("{:?}", why)
                        };
                        reply(&ctx, &msg, &"Confirmed!".to_string()).await;
                    }
                    false => {}
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