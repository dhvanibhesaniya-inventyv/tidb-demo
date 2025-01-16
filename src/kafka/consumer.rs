use std::{collections::HashMap, sync::Arc};

use async_std::stream::StreamExt;
use log::{debug, error, info, warn};
use rdkafka::{
    consumer::{BaseConsumer, Consumer, ConsumerContext, Rebalance, StreamConsumer}, error::KafkaError, message::BorrowedMessage, types::RDKafkaErrorCode, ClientConfig, ClientContext, Message, Offset
};
use lazy_static::lazy_static;
use crate::configuration;
lazy_static! {
    pub static ref MY_CONSUMER: CurrentStreamConsumer = CurrentStreamConsumer::new(&configuration::get::<String>("kafka.consumergroup"),&configuration::get::<String>("kafka.consumertopic")).unwrap();
}
#[derive(Clone)]
pub struct CurrentStreamConsumer {
    pub topic_name: String,
    pub group_name: String,
    pub client_config: ClientConfig,
    pub consumer: Arc<StreamConsumer<ConsumerCallbackLogger>>,
}
impl CurrentStreamConsumer {
    pub fn new(group_name: &str, topic_name: &str) -> Result<CurrentStreamConsumer, KafkaError> {
        let new_consumer = Self::create_stream_consumer(group_name, topic_name);
        match new_consumer {
            Ok((stream_consumer, client_config)) => {
                let consumer_object = CurrentStreamConsumer {
                    topic_name: topic_name.to_string(),
                    group_name: group_name.to_string(),
                    client_config,
                    consumer: Arc::new(stream_consumer),
                };
                return Ok(consumer_object);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    pub async fn consume_msg(&self, batch_size: usize) -> Result<BorrowedMessage, String> {
        let m: Option<Result<rdkafka::message::BorrowedMessage<'_>, KafkaError>> = self.consumer.stream().take(batch_size as usize).next().await;
        match m {
            Some(result) => match result {
                Ok(borrow_msg_obj1) => {
                    if borrow_msg_obj1.payload().is_some() {
                        log::info!("\nMSG From kafka Consumer: {:?}", String::from_utf8(borrow_msg_obj1.payload().unwrap().to_vec()));
                    }
                    return Ok(borrow_msg_obj1);
                }
                Err(err) => {
                    if err.rdkafka_error_code().is_some() {
                        self.handle_consumer_error(err.rdkafka_error_code().unwrap());
                    }
                    return Err(err.to_string());
                }
            },
            None => {
                let err = format!("Consumer can not find any result of borrow msg, In File:{:?} ,on line:{:?} ", file!(), line!());
                log::error!("{}", err);
                return Err(err);
            }
        }
    }

    fn handle_consumer_error(&self, error: RDKafkaErrorCode) {
        match &error {
            RDKafkaErrorCode::UnknownTopicOrPartition => {
                log::error!("UnknownTopicOrPartition Error: {} in {} on line:{}", error, file!(), line!());
            }
            RDKafkaErrorCode::BrokerTransportFailure => {
                log::error!("BrokerTransportFailure Error:{} in {} on line:{}", error, file!(), line!());
            }
            _ => {
                log::error!("Unhandled Kafka error: {} in {} on line:{}", error, file!(), line!());
                // Handle other unhandled errors as needed
            }
        }
    }

    pub fn process_data<F>(&self, closure: F, msg: BorrowedMessage) -> String
    where
        F: Fn(BorrowedMessage) -> String,
    {
        closure(msg)
    }

    fn create_stream_consumer(group_name: &str, topic_name: &str) -> Result<(StreamConsumer<ConsumerCallbackLogger>, ClientConfig), KafkaError> {
        let client_config = Self::set_new_consumer_config(group_name, 10);
        let connection: Result<StreamConsumer<ConsumerCallbackLogger>, KafkaError> = client_config.create_with_context(ConsumerCallbackLogger {});
        match connection {
            Ok(stream_consumer) => {
                let final_connection = stream_consumer.subscribe(&[&topic_name]);
                match final_connection {
                    Ok(_) => {
                        log::info!("kafka Consumer created Successfully with topic name:{} and group name: {}", topic_name, group_name);
                        return Ok((stream_consumer, client_config));
                    }
                    Err(kafka_error) => {
                        log::error!("Kafka Error: {} in file: {} on line: {}", kafka_error, file!(), line!());
                        return Err(kafka_error);
                    }
                }
            }
            Err(kafka_error) => {
                log::error!("Kafka Error: {} in {} on line : {}", kafka_error, file!(), line!());
                return Err(kafka_error);
            }
        }
    }
    fn set_new_consumer_config(group_name: &str, _batch_size: usize) -> ClientConfig {
        let mut config = ClientConfig::new();
        if !group_name.is_empty() {
            let config_map = Self::consumer_config(group_name);

            for (k, v) in config_map.iter() {
                config.set(k, v);
            }
        } else {
            let config_map = Self::consumer_config("");
            for (k, v) in config_map.iter() {
                config.set(k, v);
            }
        }
        config
    }
    fn consumer_config(mut group_name: &str) -> HashMap<String, String> {
        let mut tamp: HashMap<&str, &str> = HashMap::new();
        if group_name.is_empty() {
            group_name = configuration::get(&"kafka.consumergroup");
        }
        let bootstrap_server = configuration::get::<String>(&"kafka.bootstrapserver");
        let auto_offset_reset = configuration::get::<String>(&"kafka.autoOffsetReset");
        let connect_timeout = configuration::get::<String>(&"kafka.connectTimeout");
        let auto_commit = configuration::get::<String>(&"kafka.autoCommit");
        tamp.insert("bootstrap.servers", bootstrap_server.as_str());
        tamp.insert("group.id", group_name);
        tamp.insert("auto.offset.reset", auto_offset_reset.as_str());
        tamp.insert("enable.auto.commit", auto_commit.as_str()); // Enable automatic offset commits
        tamp.insert("fetch.wait.max.ms", connect_timeout.as_str());
        return tamp.iter().map(|(&k, &v)| (k.to_string(), v.to_string())).collect();
    }

    // fn consumer_config(mut group_name: &str) -> HashMap<String, String> {
    //     let mut tamp: HashMap<&str, &str> = HashMap::new();
    //     if group_name.is_empty() {
    //         group_name = configuration::get(&"kafka.consumergroup");
    //     }
    //     let bootstrap_server = configuration::get::<String>(&"kafka.bootstrapserver");
    //     let auto_offset_reset = configuration::get::<String>(&"kafka.autoOffsetReset");
    //     let connect_timeout = configuration::get::<String>(&"kafka.connectTimeout");
    //     let auto_commit = configuration::get::<String>(&"kafka.autoCommit");
    //     tamp.insert("bootstrap.servers", bootstrap_server.as_str());
    //     tamp.insert("sasl.mechanism", "SCRAM-SHA-256");
    //     tamp.insert("security.protocol", "sasl_ssl");
    //     tamp.insert("ssl.ca.location", "/home/inventyv/Downloads/ssl.ca");
    //     tamp.insert("group.id", group_name);
    //     tamp.insert("sasl.username", "kafka-scram-client-credential");
    //     tamp.insert("sasl.password", "MWM1Nl97hsCEp3XghfENTD1dla7mrIBu");
    //     tamp.insert("auto.offset.reset", auto_offset_reset.as_str());
    //     tamp.insert("enable.auto.commit", auto_commit.as_str()); // Enable automatic offset commits
    //     tamp.insert("fetch.wait.max.ms", connect_timeout.as_str());
    //     return tamp.iter().map(|(&k, &v)| (k.to_string(), v.to_string())).collect();
    // }
    
}
pub struct ConsumerCallbackLogger;

impl ClientContext for ConsumerCallbackLogger {
    const ENABLE_REFRESH_OAUTH_TOKEN: bool = false;

    fn log(&self, level: rdkafka::config::RDKafkaLogLevel, fac: &str, log_message: &str) {
        match level {
            rdkafka::config::RDKafkaLogLevel::Emerg | rdkafka::config::RDKafkaLogLevel::Alert | rdkafka::config::RDKafkaLogLevel::Critical | rdkafka::config::RDKafkaLogLevel::Error => {
               error!(target: "librdkafka", "librdkafka: {} {}", fac, log_message)
            }
            rdkafka::config::RDKafkaLogLevel::Warning => {
                warn!(target: "librdkafka", "librdkafka: {} {}", fac, log_message)
            }
            rdkafka::config::RDKafkaLogLevel::Notice => {
               info!(target: "librdkafka", "librdkafka: {} {}", fac, log_message)
            }
            rdkafka::config::RDKafkaLogLevel::Info => {
               info!(target: "librdkafka", "librdkafka: {} {}", fac, log_message)
            }
            rdkafka::config::RDKafkaLogLevel::Debug => {
                debug!(target: "librdkafka", "librdkafka: {} {}", fac, log_message)
            }
        }
    }

    fn stats(&self, statistics: rdkafka::Statistics) {
       info!("Client stats: {:?}", statistics);
    }

    fn stats_raw(&self, statistics: &[u8]) {
        match serde_json::from_slice(statistics) {
            Ok(stats) => self.stats(stats),
            Err(e) =>error!("Could not parse statistics JSON: {}", e),
        }
    }

    fn error(&self, error: rdkafka::error::KafkaError, reason: &str) {
       error!("librdkafka: {}: {}", error, reason);
    }

    fn generate_oauth_token(&self, _oauthbearer_config: Option<&str>) -> Result<rdkafka::client::OAuthToken, Box<dyn std::error::Error>> {
        Err("Default implementation of generate_oauth_token must be overridden".into())
    }
}

impl ConsumerContext for ConsumerCallbackLogger {
    fn pre_rebalance<'a>(&self, _rebalance: &rdkafka::consumer::Rebalance<'a>) {}

    fn post_rebalance<'a>(&self, rebalance: &rdkafka::consumer::Rebalance<'a>) {
        log::info!("post_rebalance callback");

        match rebalance {
            Rebalance::Assign(tpl) => {
                for e in tpl.elements() {
                    log::info!("rebalanced partition {}", e.partition())
                }
            }

            Rebalance::Error(err_info) => {
                log::error!("Post Rebalance error {}", err_info)
            }
            Rebalance::Revoke(_) => {}
        }
    }

    fn commit_callback(&self, result: rdkafka::error::KafkaResult<()>, offsets: &rdkafka::TopicPartitionList) {
        match result {
            Ok(_) => {
                for e in offsets.elements() {
                    match e.offset() {
                        //skip Invalid offset
                        Offset::Invalid => {}
                        _ => {
                            // log::info!("committed offset {:?} in partition {}", e.offset(), e.partition(),)
                        }
                    }
                }
            }
            Err(err) => {
                log::info!("error committing offset - {}", err)
            }
        }
    }
}

pub struct CurrentBaseConsumer {
    pub topic_name: String,
    pub group_name: String,
    pub consumer: Arc<BaseConsumer<ConsumerCallbackLogger>>,
}
impl CurrentBaseConsumer {
    pub fn new(topic_name: String, group_name: String, consumer: BaseConsumer<ConsumerCallbackLogger>) -> Self {
        CurrentBaseConsumer { topic_name, group_name, consumer: Arc::new(consumer) }
    }
}
