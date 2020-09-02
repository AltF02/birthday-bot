use serenity::{
    async_trait,
    model::prelude::*,
    prelude::*,
};


pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        let user = &ready.user;
        println!("Logged in as {}", user.name)
    }
}