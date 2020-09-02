mod config;
mod bot;

use crate::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::new(String::from("./config.yml"));
    bot::start(config).await;
}
