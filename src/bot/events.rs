use serenity::{
    async_trait,
    model::prelude::*,
    prelude::*,
};

use log::{
    info,
    warn
};

use tokio_postgres::{
    Client as DBClient
};
use crate::database::DataBase;
use crate::config::Config;
use crate::bot::utils::{check_msg, calculate_age};
use chrono::NaiveDate;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let user = &ready.user;
        ctx.set_presence(Some(Activity::listening("to your birthday songs")), OnlineStatus::Online).await;
        let data = ctx.data.read().await;
        let db = data.get::<DataBase>().unwrap();
        let config = data.get::<Config>().unwrap();
        info!("Logged in as {}#{}", user.name, user.discriminator);
        check_birthday(db, config, &ctx).await;
    }
}

#[allow(unused_must_use)]
async fn check_birthday(db: &DBClient, config: &Config, ctx: &Context) {
    loop {
        let birthdays = db.query("SELECT date, user_id FROM birthdaybot.birthdays WHERE has_role = false AND date = date(now())", &[]).await.unwrap();
        let guild = ctx.http.get_guild(config.guild_id).await.unwrap();

        if !birthdays.is_empty() {
            for birthday in birthdays {
                let user_id: i64 = birthday.get(1);
                if let Ok(mut member) = guild.member(ctx, UserId(user_id as u64)).await {
                    if let Err(e) = member.add_role(&ctx.http, RoleId(config.role_id)).await {
                        warn!("Cannot give role: {}", e)
                    } else {
                        info!("Gave role to {}", member.user.name)
                    }

                    db.query("UPDATE birthdaybot.birthdays SET has_role = true WHERE user_id = $1",
                             &[&(member.user.id.0 as i64)]).await;

                    let channel = ctx.http.get_channel(752960824229757078).await.unwrap();
                    let date: NaiveDate = birthday.get(0);

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

        let birthdays_over = db
            .query("SELECT date, user_id FROM birthdaybot.birthdays WHERE has_role = true AND date != date(now())", &[])
            .await
            .unwrap();

        for birthday_over in birthdays_over {
            let user_id: i64 = birthday_over.get(1);
            if let Ok(mut member) = guild.member(ctx, UserId(user_id as u64)).await {
                db.query("UPDATE birthdaybot.birthdays SET has_role = false WHERE user_id = $1",
                         &[&(member.user.id.0 as i64)]).await;

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