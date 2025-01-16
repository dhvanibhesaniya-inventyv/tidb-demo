use config::Config;
pub mod settings;
use lazy_static::lazy_static;
// use crate::error::{ConfigError, Result};
use serde::de::Deserialize;
use std::sync::RwLock;
lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(settings::get_config());
}

// pub fn get(property:String) -> String{
//     CONFIG.lock().unwrap().get(&property).unwrap()
// }

pub fn get<'de, T: Deserialize<'de>>(key: &str) -> T {
    // println!("{:?}",key);
    // println!("{:?}",CONFIG.read().unwrap());
    CONFIG.read().unwrap().get(key).unwrap()
}

// pub async fn initialize_config() {
//     let config_result = settings::get_config();
//     let mut m_config = CONFIG.write().unwrap();
//     *m_config = config_result;
//     // match config_result {
//     //     Ok(config) => {
//     //         let mut m_config = CONFIG.write().unwrap();
//     //         *m_config = config;
//     //         println!("New Config Loaded");
//     //         return Ok(());
//     //     }
//     //     Err(error) => {
//     //         println!("Error While loading config from TIKV Error :-{:?}", error.to_string());
//     //         return Err(error);
//     //     }
//     // }
// }
