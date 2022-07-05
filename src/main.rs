mod core;
mod bot;

use crate::bot::config::config_service_impl::ConfigServiceImpl;
use crate::bot::handler::{BotState, Handler};
use crate::core::config::config_service::ConfigService;

use serenity::prelude::*;

#[tokio::main]
async fn main() {
    // initialisation des services
    let config_service = ConfigServiceImpl::new();

    // recuperation du token de bot
    let token = config_service.get_token();
    println!("{}", token);

    // les events que le bot écoute
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // creation du bot
    let mut client =
        Client::builder(&token, intents)
            .event_handler(Handler {state: BotState::Nothing})
            .await
            .expect("Err creating client");

    // on demarre le bot
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}