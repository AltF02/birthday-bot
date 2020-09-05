mod events;
mod commands;
mod utils;
mod checks;

use crate::config::Config;
use crate::database;

use events::Handler;
use serenity::{
    framework::standard::{
        StandardFramework,
    },
    prelude::TypeMapKey,
    Client,
    client::bridge::gateway::ShardManager,
};

use crate::database::DataBase;
use serenity::http::Http;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

impl TypeMapKey for Config {
    type Value = Config;
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub async fn start(config: Config) {

    let http = Http::new_with_token(&config.token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }

        Err(why) => panic!("Could not access app info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix(&config.prefix);
            c.on_mention(Some(bot_id));
            c.allow_dm(true);
            c.case_insensitivity(true);
            c.owners(owners);
            return c;
        })
        .group(&commands::GENERAL_GROUP)
        .group(&commands::OWNER_GROUP);

    let mut client = Client::new(&config.token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let db_client = database::connect(&config.db_uri).await;

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(config);
        data.insert::<DataBase>(db_client);
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(e) = client.start().await {
        panic!("Failed to start bot \n{:?}", e)
    }
}