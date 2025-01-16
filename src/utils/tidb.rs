use chrono::Utc;
use lazy_static::lazy_static;
use mysql_async::{prelude::Queryable, Opts, Pool};
use mysql_async::{Row, Value as MysqlValue};
use serde_json::{json, Value};
use tokio::task;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::configuration;
// use crate::utils::common::produce_kafka_msg;

lazy_static! {
    static ref CONNECTION_POOLS: Arc<RwLock<HashMap<String, Pool>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

fn get_or_init_connection_pool(db_name: &str) -> Pool {
    let mut pools = CONNECTION_POOLS.write().unwrap();
    if let Some(pool) = pools.get(db_name) {
        log::info!(
            "Already exits connection pool for database {:?} Returning existsing pool",
            db_name
        );
        pool.clone()
    } else {
        let pool_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            configuration::get::<String>("tidb.username"),
            configuration::get::<String>("tidb.password"),
            configuration::get::<String>("tidb.host"),
            configuration::get::<String>("tidb.port"),
            db_name
        );
        println!("{:?}", pool_url);
        let opts = Opts::from_url(pool_url.as_str()).expect("Failed to parse database URL");
        let pool = Pool::new(opts);
        pools.insert(db_name.to_string(), pool.clone());
        log::info!("Created connection pool for database {:?} Inserting same into pool map and returing it",db_name);
        pool
    }
}

pub async fn execute_query(query: String, db_name: String) -> Result<(String, String), String> {
    //get connection pool object
    let pool = get_or_init_connection_pool(&db_name);
    //get connection from pool
    let conn = pool.get_conn().await;
    match conn {
        Ok(mut conn) => {
            let timestamp = if query.to_lowercase().starts_with("update") && query.contains("where") { Utc::now().format("%Y-%m-%d %H:%M:%S").to_string() } else { "".to_string()};
            //execute query
            let query_result: Result<Vec<Row>, mysql_async::Error> =
                conn.query(query.to_owned()).await;
            // conn.
            match query_result {
                Ok(rows) => {
                    // convert response rows to json value
                    log::info!("rows ---------------------------------------> {:?}", rows);
                    let mut json_array = Vec::new();
                    for row in rows {
                        json_array.push(convert_row_to_json(row));
                    }
                    if conn.affected_rows() > 0 && query.to_lowercase().starts_with("update") && query.to_lowercase().contains("where")  {
                        task::spawn(async move {
                        //     produce_kafka_msg(timestamp, query,db_name).await;
                        });
                    }
                    Ok((
                        format!("Query OK,{} row affected", conn.affected_rows()),
                        serde_json::to_string(&json_array).unwrap(),
                    ))
                }
                Err(error) => {
                    log::error!(
                        "Error while executing query {:?} with db name {:?} Error:-{:?}",
                        query,
                        db_name,
                        error.to_string()
                    );
                    Err(error.to_string())
                }
            }
        }
        Err(error) => {
            log::error!("Error while connection with database Error:-{:?}", error);
            Err(error.to_string())
        }
    }
}
pub async fn check_connection(db_name: String) -> Result<bool, String> {
    //get connection pool object
    let pool = get_or_init_connection_pool(&db_name);
    //get connection from pool
    let conn = pool.get_conn().await;
    match conn {
        Ok(mut conn) => {
            //execute query
            let result = conn.ping().await;
            // conn.
            match result {
                Ok(_) => {
                    log::info!("Connection with db{:?} working fine", db_name);
                    Ok(true)
                }
                Err(error) => {
                    log::error!(
                        "Error in connection with db {:?} Error:-{:?}",
                        db_name,
                        error.to_string()
                    );
                    Err(error.to_string())
                }
            }
        }
        Err(error) => {
            log::error!("Error while connection with database Error:-{:?}", error);
            Err(error.to_string())
        }
    }
}

/// this function will convert response from qeury to json
fn convert_row_to_json(row: Row) -> Value {
    let mut json_obj = serde_json::Map::new();
    for (i, col) in row.columns().iter().enumerate() {
        let column_name = col.name_str().to_owned();
        let column_value = match row.get_opt(i) {
            Some(Ok(mysql_value)) => match mysql_value {
                MysqlValue::Bytes(bytes) => {
                    // Convert bytes to UTF-8 string
                    String::from_utf8_lossy(&bytes).into_owned()
                }
                MysqlValue::NULL => String::new(), // Return empty string for NULL values
                _ => format!("{:?}", mysql_value), // Convert other types to string
            },
            _ => String::new(), // Return empty string for errors or None values
        };
        json_obj.insert(column_name.to_string(), json!(column_value));
    }
    Value::Object(json_obj)
}