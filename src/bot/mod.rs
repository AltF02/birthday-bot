mod events;
mod commands;
mod utils;

use crate::config::Config;

use events::Handler;
use serenity::{
    framework::standard::{
        StandardFramework,
    },
    prelude::TypeMapKey,
    Client
};

impl TypeMapKey for Config {
    type Value = Config;
}

pub async fn start(config: Config) {
    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix(&config.prefix);
            c.allow_dm(true);
            c.case_insensitivity(true);
            return c;
        })
        .group(&commands::COMMANDS_GROUP);

    let mut client = Client::new(&config.token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(config);
    }

    if let Err(e) = client.start().await {
        panic!("Failed to start bot \n{:?}", e)
    }
}