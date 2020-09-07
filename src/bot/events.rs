use serenity::{
    async_trait,
    model::prelude::*,
    prelude::*,
};

use log::{
    info
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let user = &ready.user;
        ctx.set_presence(Some(Activity::listening("to your birthday songs")), OnlineStatus::Online).await;
        info!("Logged in as {}#{}", user.name, user.discriminator)
    }
}