use std::collections::HashMap;
use config::Config;

use crate::core::config::config_service::ConfigService;

pub struct ConfigServiceImpl {
    configs: HashMap<String, String>
}

impl ConfigServiceImpl {
    pub fn new() -> ConfigServiceImpl {
        // setting
        let config = Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()
            .unwrap();
        let configs = config
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();

        ConfigServiceImpl {configs}
    }
}

impl ConfigService for ConfigServiceImpl {
    fn get_token(&self) -> String {
        self.configs.get("bot_token").unwrap().clone()
    }
}