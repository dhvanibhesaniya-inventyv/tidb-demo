use chrono::DateTime;
use chrono::Utc;
use lazy_static::lazy_static;
use serde_json::json;
use serde_json::Value;
use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlRow;
use sqlx::Column;
use sqlx::Row;
use sqlx::TypeInfo;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

lazy_static! {
    static ref CONNECTION_POOLS: Arc<RwLock<HashMap<String, MySqlPool>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

pub async fn get_or_init_connection_pool(db_name: &str) -> MySqlPool {
    let mut pools = CONNECTION_POOLS.write().unwrap();
    if let Some(pool) = pools.get(db_name) {
        log::info!(
            "Already exists connection pool for database {:?}. Returning existing pool",
            db_name
        );
        pool.clone()
    } else {
        let pool_url = format!("mysql://root:@127.0.0.1:4000/{}", db_name);
        println!("Connecting to database: {:?}", pool_url);
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&pool_url)
            .await
            .expect("Failed to create connection pool");
        pools.insert(db_name.to_string(), pool.clone());
        log::info!(
            "Created connection pool for database {:?}. Inserting into pool map and returning it",
            db_name
        );
        pool
    }
}

pub async fn execute_query(query: String, db_name: String) -> Result<(String, String), String> {
    let pool = get_or_init_connection_pool(&db_name).await;

    // Execute the query and fetch all results
    match sqlx::query(&query).fetch_all(&pool).await {
        Ok(rows) => {
            let timestamp = if query.to_lowercase().starts_with("update") && query.contains("where")
            {
                Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
            } else {
                "".to_string()
            };

            log::info!("rows ---------------------------------------> {:?}", rows);

            // Convert response rows to JSON value
            let json_array: Vec<Value> = rows.into_iter().map(convert_row_to_json).collect();

            if query.to_lowercase().starts_with("update") && query.contains("where") {
                // Uncomment and implement Kafka producer logic if required
                // task::spawn(async move {
                //     produce_kafka_msg(timestamp, query, db_name).await;
                // });
            }

            Ok((
                format!("Query OK, {} rows affected", json_array.len()),
                serde_json::to_string(&json_array).unwrap(),
            ))
        }
        Err(err) => {
            log::error!(
                "Error while executing query {:?} with db name {:?}. Error: {:?}",
                query,
                db_name,
                err
            );
            Err(err.to_string())
        }
    }
}

pub async fn check_connection(db_name: String) -> Result<bool, String> {
    let pool = get_or_init_connection_pool(&db_name).await;

    // Try to acquire a connection from the pool
    match pool.acquire().await {
        Ok(mut conn) => {
            // Execute a simple query to test the connection
            match sqlx::query("SELECT 1").execute(&mut *conn).await {
                Ok(_) => {
                    log::info!("Connection with database {:?} is working fine.", db_name);
                    Ok(true)
                }
                Err(err) => {
                    log::error!(
                        "Error while testing connection with database {:?}. Error: {:?}",
                        db_name,
                        err
                    );
                    Err(err.to_string())
                }
            }
        }
        Err(err) => {
            log::error!(
                "Error while acquiring connection from the pool for database {:?}. Error: {:?}",
                db_name,
                err
            );
            Err(err.to_string())
        }
    }
}

/// Converts a row from SQLx into a JSON object.

fn convert_row_to_json(row: MySqlRow) -> Value {
    let mut json_obj = serde_json::Map::new();

    for col in row.columns() {
        let column_name = col.name().to_owned();

        // Match column type dynamically
        let column_value = match col.type_info().name() {
            "INT" | "BIGINT" | "SMALLINT" | "TINYINT" => row
                .try_get::<i32, _>(column_name.as_str())
                .ok()
                .map_or(json!(null), |a| json!(a)),
            "FLOAT" => row
                .try_get::<f32, _>(column_name.as_str())
                .ok()
                .map_or(json!(null), |b| json!(b)),
            "VARCHAR" | "TEXT" | "CHAR" => row
                .try_get::<String, _>(column_name.as_str())
                .ok()
                .map_or(json!(null), |c| json!(c)),
            "JSON" => row
                .try_get::<serde_json::Value, _>(column_name.as_str())
                .ok()
                .map_or(json!(null), |d| json!(d.to_string())),
            "BOOLEAN" => row
                .try_get::<bool, _>(column_name.as_str())
                .ok()
                .map_or(json!(null), |e| json!(e)),
            "DATETIME" | "TIMESTAMP" => {
                // Try to get as chrono::NaiveDateTime first
                match row.try_get::<DateTime<Utc>, _>(column_name.as_str()) {
                    Ok(dt) => {
                        json!(dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    }
                    Err(_err) => {
                        // Fallback to string if datetime parsing fails
                        row.try_get::<String, _>(column_name.as_str())
                            .ok()
                            .map_or(json!(null), |f| json!(f))
                    }
                }
            }
            _ => {
                // Try to get as string first for unknown types
                row.try_get::<String, _>(column_name.as_str())
                    .ok()
                    .map_or(json!(null), |val| json!(val))
            }
        };

        json_obj.insert(column_name, column_value);
    }

    Value::Object(json_obj)
}

// fn convert_row_to_json(row: sqlx::mysql::MySqlRow) -> Value {
//     let mut json_obj = serde_json::Map::new();

//     for (i, col) in row.columns().iter().enumerate() {
//         let column_name = col.name().to_owned();

//         // Retrieve column value using try_get for better error handling
//         let column_value: String = match row.try_get(i) {
//             Ok(value) => value,
//             Err(_) => String::new(), // Handle errors gracefully
//         };

//         json_obj.insert(column_name.to_string(), json!(column_value));
//     }

//     Value::Object(json_obj)
// }

// fn convert_row_to_json(row: sqlx::mysql::MySqlRow) -> Value {
//     let mut json_obj = serde_json::Map::new();

//     for (i, col) in row.columns().iter().enumerate() {
//         let column_name = col.name().to_owned();
//         println!("column_name ------------------------- {:?}",column_name);

//         // Get the value for the column and handle different types accordingly
//         let column_value = match row.try_get::<String, _>(i) {
//             Ok(value) => value,
//             Err(_) => String::new(), // Return empty string for errors or None values
//         };

//         json_obj.insert(column_name.to_string(), json!(column_value));
//     }

//     Value::Object(json_obj)
// }
