#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_unsafe)]

use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use log4rs::append::rolling_file::policy::compound::{
    roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
};
use log4rs::append::{
    console::ConsoleAppender, file::FileAppender, rolling_file::RollingFileAppender,
};
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::json::{JsonEncoder, JsonEncoderConfig};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::encode::writer::simple::SimpleWriter;
use log4rs::encode::Style;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Config;
// use log4rs::*;

use chrono::{DateTime, Local};
use log::{Level, LevelFilter};

use crate::configuration;

lazy_static! {
    //Most of these will define in config File.
    // static ref LOG_LINE_PATTERN_CONSOLE: &'static str = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";
    // static ref LOG_LINE_PATTERN_FILE: &'static str = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";

    static ref TRIGGER_FILE_SIZE : u64 = configuration::get::<u64>(&"logger.logfilesize".to_string());
    static ref ROLLER_FILEPATH_PATTERN: String  = configuration::get::<String>(&"logger.rollerfilepathpattern".to_string());
    static ref ROLLER_MAX_COUNT: u32 = configuration::get::<u32>(&"logger.rollermaxcount".to_string()) as u32;
    static ref ROLLER_BASE_START: u32 = configuration::get::<u32>(&"logger.rollerbasestart".to_string()) as u32;
    static ref ALL_LOG_FILE_PATH:String = configuration::get::<String>(&"logger.alllogscommonfilepath".to_string());
}

pub struct RollingFileAppenderComponent {
    pub trigger: Box<SizeTrigger>,
    pub roller: Box<FixedWindowRoller>,
    pub compound_policy: Box<CompoundPolicy>,
}
impl RollingFileAppenderComponent {
    pub fn new() -> Box<CompoundPolicy> {
        //Set trigger, roller and compoundPolicy for "RollingFileAppender"
        let trigger = Box::new(SizeTrigger::new(*TRIGGER_FILE_SIZE));
        let roller = Box::new(
            FixedWindowRoller::builder()
                .base(*ROLLER_BASE_START)
                .build(&ROLLER_FILEPATH_PATTERN, *ROLLER_MAX_COUNT)
                .unwrap(),
        );
        let compound_policy = Box::new(CompoundPolicy::new(trigger.clone(), roller.clone()));
        return compound_policy;
    }
}

pub struct LoggerConfig {}
impl LoggerConfig {
    pub fn create_Global_logs_config() -> Config {
        //===== get compound Policy =====
        let compound_policy: Box<CompoundPolicy> = RollingFileAppenderComponent::new();

        //===== set appanders for console and file =====
        let console_appender = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{l} - {m}{n}")))
            .build();

        //pattern vise logs
        // let all_log_appender = RollingFileAppender::builder()
        // .encoder(Box::new(PatternEncoder::new(&LOG_LINE_PATTERN_FILE)))
        // .build(*ALL_LOG_FILE_PATH, compound_policy)
        // .unwrap();

        //json format logs
        let all_log_appender = RollingFileAppender::builder()
            .encoder(Box::new(JsonEncoder::new()))
            .build(*&ALL_LOG_FILE_PATH.as_str(), compound_policy)
            .unwrap();

        //===== create config =====
        //ThresholdFilter is mendatory for set LogLevel on specific appenders
        //describe all appenders in config. and declare only that appanders name in Root which you want to use.
        //set your Max(Default) Log-level in Root.

        // TO-DO : replace level og log

        let Global_logs_config = Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(LevelFilter::Debug)))
                    .build("console_appender", Box::new(console_appender)),
            )
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(LevelFilter::Debug)))
                    .build("all_log_appender", Box::new(all_log_appender)),
            )
            .logger(
                Logger::builder()
                    .appender("all_log_appender")
                    .additive(true)
                    .build("All-Logs", LevelFilter::Debug),
            )
            .logger(
                Logger::builder()
                    .appender("console_appender")
                    .additive(true)
                    .build("console", LevelFilter::Debug),
            )
            .build(
                Root::builder()
                    .appenders(["console_appender", "all_log_appender"])
                    .build(LevelFilter::Trace),
            )
            .unwrap();

        // let Global_logs_config = Config::builder();
        // if config::get("logLevel") == "error"{

        // }
        Global_logs_config
        // Global_logs_config
    }
}

pub fn startLogger() {
    // You can use handle to change logger config at runtime
    // just call startLogger() in main.rs and you can use log4rs in all your Project-crate.
    let Global_logs_config = LoggerConfig::create_Global_logs_config();
    // println!("{:?}", Global_logs_config);
    let handle = log4rs::init_config(Global_logs_config).unwrap();
}
