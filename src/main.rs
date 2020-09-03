mod config;
mod bot;
mod database;

use crate::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::new(String::from("./config.yml"));
    bot::start(config).await;
}
