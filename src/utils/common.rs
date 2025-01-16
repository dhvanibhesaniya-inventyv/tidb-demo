use serde::{Deserialize, Serialize};

// use crate::{configuration, kafka::producer::MY_PRODUCER};

// pub async fn produce_kafka_msg(timestamp:String,query: String,db_name: String) {
//     let msg = KafkaMsg { timestamp , query ,db_name};
//     let kafka_res = MY_PRODUCER.send_message(configuration::get::<String>("kafka.rowhistory"),serde_json::to_string(&msg).unwrap(),"".to_string()).await;
//     match kafka_res  {
//         Ok(()) => {
//             log::info!("Msg Produced Successfully for update");
//         }
//         Err(error)=>{
//             log::error!("kafka error while producing msg {:?}",error.to_string());
//         }
//     }
// }




#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KafkaMsg {
    pub query: String,
    pub timestamp: String,
    pub db_name:String
}