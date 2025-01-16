use rdkafka::{producer::ProducerContext, ClientContext};
use crate::configuration;
use lazy_static::lazy_static;
use rdkafka::{
    error::KafkaError,
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};

use std::{collections::HashMap, sync::Arc, time::Duration};
lazy_static! {
    pub static ref MY_PRODUCER: CurrentProducer = CurrentProducer::new().unwrap();
}

#[derive(Clone)]
pub struct CurrentProducer {
    pub producer: Arc<FutureProducer<ProduceCallbackLogger>>,
}

impl CurrentProducer {
    pub fn new() -> Result<CurrentProducer, KafkaError> {
        let new_producer_result: Result<FutureProducer<ProduceCallbackLogger>, KafkaError> = Self::create_producer();
        match new_producer_result {
            Ok(new_producer) => {
                let producer_obj = CurrentProducer { producer: new_producer.into() };
                return Ok(producer_obj);
            }
            Err(kafka_error) => {
                return Err(kafka_error);
            }
        }
    }
    fn create_producer() -> Result<FutureProducer<ProduceCallbackLogger>, KafkaError> {
        let mut config = ClientConfig::new();
        let config_map = Self::producer_config();
        for (k, v) in config_map.iter() {
            config.set(k, v);
        }
        let connection: Result<FutureProducer<ProduceCallbackLogger>, KafkaError> = config.create_with_context(ProduceCallbackLogger {});
        match connection {
            Ok(connection) => return Ok(connection),
            Err(kafkaerror) => {
                log::error!("Error:{}", kafkaerror);
                return Err(kafkaerror);
            }
        }
    }
    fn producer_config() -> HashMap<String, String> {
        let mut config_producer = HashMap::new();
        let bootstrap_server = configuration::get::<String>(&"kafka.bootstrapserver");
        // let _connect_timeout = config::get::<String>(&"kafka.connectTimeout");
        config_producer.insert("bootstrap.servers".to_string(), bootstrap_server);
        return config_producer;
    }
    
    pub async fn send_message(&self, topic_name: String, payload: String,key: String) -> Result<(), KafkaError> {
        let record = FutureRecord::to(&topic_name).payload(&payload).key(&key);
        let send_data = self.producer.send(record, Duration::from_micros(100)).await;
        match send_data {
            Ok((partition_number, offset_number)) => {
                log::info!("message has been send to partition: {} and offset: {} ", partition_number, offset_number);
                return Ok(());
            }
            Err((kafka_error, _owned_massage)) => {
                //_owned_massage : owned copy of the original message
                return Err(kafka_error);
            }
        }
        // TO-DO : decide duration
    }
}

pub struct ProduceCallbackLogger;
impl ClientContext for ProduceCallbackLogger {}
impl ProducerContext for ProduceCallbackLogger {
    type DeliveryOpaque = ();

    fn delivery(&self, delivery_result: &rdkafka::producer::DeliveryResult<'_>, _delivery_opaque: Self::DeliveryOpaque) {
        let dr = delivery_result.as_ref();
        match dr {
            Ok(msg) => {
                let key: &str = msg.key_view().unwrap().unwrap();
                log::info!("produced message with key {} in offset {} of partition {}", key, msg.offset(), msg.partition())
            }
            Err(producer_err) => {
                let key: &str = producer_err.1.key_view().unwrap().unwrap();
                log::error!("failed to produce message with key {} - {}", key, producer_err.0,)
            }
        }
    }
}
