use serenity::{async_trait, model::prelude::*, prelude::*};

use log::{info, warn};

use crate::bot::utils::{calculate_age, check_msg};
use crate::config::Config;
use crate::database::ConnectionPool;
use chrono::NaiveDate;
use sqlx;
use sqlx::PgPool;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let user = &ready.user;
        ctx.set_presence(
            Some(Activity::listening("your birthday songs")),
            OnlineStatus::Online,
        )
        .await;
        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().unwrap();
        let config = data.get::<Config>().unwrap();
        info!("Logged in as {}#{}", user.name, user.discriminator);
        check_birthday(pool, config, &ctx).await;
    }
}

#[allow(unused_must_use)]
async fn check_birthday(pool: &PgPool, config: &Config, ctx: &Context) {
    loop {
        let birthdays = sqlx::query!("SELECT date, user_id FROM birthdaybot.birthdays WHERE has_role = false AND date = date(now())")
            .fetch_all(pool)
            .await
            .unwrap();

        let guild = ctx.http.get_guild(config.guild_id).await.unwrap();

        if !birthdays.is_empty() {
            for birthday in birthdays {
                let user_id: i64 = birthday.user_id;
                if let Ok(mut member) = guild.member(ctx, UserId(user_id as u64)).await {
                    if let Err(e) = member.add_role(&ctx.http, RoleId(config.role_id)).await {
                        warn!("Cannot give role: {}", e)
                    } else {
                        info!("Gave role to {}", member.user.name)
                    }
                    let user_id = member.user.id.0 as i64;
                    sqlx::query!(
                        "UPDATE birthdaybot.birthdays SET has_role = true WHERE user_id = $1",
                        user_id
                    )
                    .execute(pool)
                    .await;

                    let channel = ctx.http.get_channel(718691884070993935).await.unwrap();
                    let date: NaiveDate = birthday.date;

                    check_msg(channel.id().send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title(format!("Happy birthday!🎉"));

                            e.description(format!("**{}** just turned **{}**!! \
                                                Happy birthday!! \
                                                Hope you have a nice birthday and enjoy your role today🎂🎉",
                                                  member.user.mention(), calculate_age(date)));

                            e.author(|a| {
                                a.icon_url(member.user.face());
                                a.name(date.format("%d %B %Y"));
                                a
                            });
                            e.colour(0xffa500);
                            e
                        });
                        m
                    }).await);
                };
            }
        }

        let birthdays_over = sqlx::query!("SELECT date, user_id FROM birthdaybot.birthdays WHERE has_role = true AND date != date(now())")
            .fetch_all(pool)
            .await
            .unwrap();

        for birthday_over in birthdays_over {
            let user_id: i64 = birthday_over.user_id;
            if let Ok(mut member) = guild.member(ctx, UserId(user_id as u64)).await {
                sqlx::query!(
                    "UPDATE birthdaybot.birthdays SET has_role = false WHERE user_id = $1",
                    user_id
                )
                .execute(pool)
                .await;

                if let Err(e) = member.remove_role(&ctx.http, RoleId(config.role_id)).await {
                    warn!("Cannot remove role: {}", e)
                } else {
                    info!("Removed role from {}", member.user.name)
                }
            }
        }

        tokio::time::delay_for(core::time::Duration::from_secs(3600)).await;
    }
}
