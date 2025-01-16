#![allow(deprecated)]
use config::{Config, Environment, File};
// use config::Config;
use std::env;
// use tonic::async_trait;

// use crate::utils::tikv;

pub fn get_config() -> Config {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "".into());
    println!("Environemnt  {:?} Started", run_mode);
    let env_settings = Environment::new();

    let s = Config::builder()
        // Start off by merging in the "default" configuration file
        .add_source(File::with_name("resources/config/config"))
        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        .add_source(
            File::with_name(&format!("resources/config/config-{}", run_mode)).required(false),
        )
        // Add in a local configuration file
        // This file shouldn't be checked in to git
        .add_source(env_settings.prefix("app").separator("_"))
        // You may also programmatically change settings
        .build()
        .unwrap();

    // Now that we're done, let's access our configuration

    // You can deserialize (and thus freeze) the entire configuration as
    s
}

// pub async fn get_async_config() -> Result<Config, Box<dyn Error>> {
//     let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
//     let env_settings = Environment::new();
//     // uncomment UserConfig if you are running in local environment

//     let s: Config = Config::builder()
//         .add_source(File::with_name("resources/config/config"))
//         .add_source(File::with_name(&format!("resources/config/config-{}", run_mode)).required(false))
//         .add_async_source(ServiceConfig { env_key: run_mode.to_string() })
//         // This file shouldn't be checked in to git
//         .add_source(env_settings.prefix("app").separator("_"))
//         .build()
//         .await?;
//     Ok(s)
// }

// #[derive(Debug)]
// struct ServiceConfig {
//     env_key: String,
// }

// #[async_trait]
// impl AsyncSource for ServiceConfig {
//     async fn collect(&self) -> Result<Map<String, config::Value>, ConfigError> {
//         let _current_env = &self.env_key;
//         println!("In Get Async config");
//         // get session doc from tikv.
//         let response = tikv::get_single_record("CONFIG".to_string(), Some("TDL".to_string())).await;
//         println!("{:?}", response);
//         match response {
//             Ok(result) => {
//                 println!("In Get Async config{:?}", result);
//                 let config_json: Map<String, config::Value> = serde_json::from_str(&result).unwrap();
//                 Ok(config_json)
//             }
//             Err(error) => Err(ConfigError::NotFound(error.error)),
//         }
//     }
// }
