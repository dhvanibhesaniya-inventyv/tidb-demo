use datalayerapi_tidb::utils::{self, logger::startLogger};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    startLogger();

    println!("Starting MySQLx/TiDB connection test...");

    // //  // Test database name
    let db_name = "test".to_string();

    // //  // Test database connection
    // println!("\nTesting database connection...");
    // match utils::sqlx::check_connection(db_name.clone()).await {
    //     Ok(true) => println!("✓ Connection successful!"),
    //     Ok(false) => println!("✗ Connection failed"),
    //     Err(e) => println!("✗ Connection error: {}", e),
    // }

    // //  // Create a test table
    // println!("\nCreating test table...");
    // let create_table_query = "
    // CREATE TABLE IF NOT EXISTS test_1_users (
    //     id INT PRIMARY KEY AUTO_INCREMENT,
    //     name VARCHAR(100),
    //     email VARCHAR(100),
    //     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    //     address JSON,
    //     is_active BOOLEAN,
    //     score FLOAT
    // )
    // "
    // .to_string();

    // match utils::sqlx::execute_query(create_table_query, db_name.clone()).await {
    //     Ok((status, data)) => println!("✓ Table creation: {} & data : {}", status, data),
    //     Err(e) => println!("✗ Table creation failed: {}", e),
    // }

    // //  // Insert test data
    // println!("\nInserting test data...");
    // let insert_queries = vec![
    //     "INSERT INTO test_1_users (name, email, address, is_active, score) VALUES ('Alice', 'alice@example.com', '{\"address\": {\"street\": \"123 Main St\", \"city\": \"Wonderland\", \"zip\": \"12345\"}}', true, 3.5)",
    //     "INSERT INTO test_1_users (name, email, address, is_active, score) VALUES ('Bob', 'bob@example.com', '{\"address\": {\"street\": \"456 Elm St\", \"city\": \"Builderland\", \"zip\": \"67890\"}}', false, 4.5)",
    //     "INSERT INTO test_1_users (name, email, address, is_active, score) VALUES ('Charlie', 'charlie@example.com', '{\"address\": {\"street\": \"789 Oak St\", \"city\": \"Coderland\", \"zip\": \"54321\"}}', true  , 5.5)",
    // ];

    // for query in insert_queries {
    //     match utils::sqlx::execute_query(query.to_string(), db_name.clone()).await {
    //         Ok((status, data)) => println!("✓ Insert: {} and data: {}", status, data),
    //         Err(e) => println!("✗ Insert failed: {}", e),
    //     }
    // }

    // Select and display data
    println!("\nRetrieving inserted data...");
    match utils::sqlx::execute_query("SELECT * FROM test_1_users".to_string(), db_name.clone())
        .await
    {
        Ok((status, data)) => {
            println!("✓ Select: {}", status);
            println!("Data retrieved:");
            let parsed: serde_json::Value = serde_json::from_str(&data)?;
            println!("{}", serde_json::to_string_pretty(&parsed)?);
        }
        Err(e) => println!("✗ Select failed: {}", e),
    }

    // // // Test update functionality
    // println!("\nTesting update functionality...");
    // match utils::sqlx::execute_query(
    //     "UPDATE test_1_users SET email = 'alice.new@example.com' WHERE name = 'Alice'".to_string(),
    //     db_name.clone(),
    // )
    // .await
    // {
    //     Ok((status, data)) => println!("✓ Update: {} and data : {}", status, data),
    //     Err(e) => println!("✗ Update failed: {}", e),
    // }

    // // // Verify update
    // println!("\nVerifying update...");
    // match utils::sqlx::execute_query(
    //     "SELECT * FROM test_1_users WHERE name = 'Alice'".to_string(),
    //     db_name.clone(),
    // )
    // .await
    // {
    //     Ok((status, data)) => {
    //         println!("✓ Select after update: {}", status);
    //         let parsed: serde_json::Value = serde_json::from_str(&data)?;
    //         println!("{}", serde_json::to_string_pretty(&parsed)?);
    //     }
    //     Err(e) => println!("✗ Select failed: {}", e),
    // }

    // Clean up (optional - uncomment to drop the table)

    // println!("\nCleaning up test table...");
    // match utils::sqlx::execute_query(
    //     "DROP TABLE test_1_users".to_string(),
    //     db_name.clone()
    // ).await {
    //     Ok((status, data)) => println!("✓ Cleanup: {} and data : {}", status, data),
    //     Err(e) => println!("✗ Cleanup failed: {}", e),
    // }

    println!("\nTest completed!");

    Ok(())
}

// -------------------------------------------------------------------------------------------------------------------------------------
//    questions
// -------------------------------------------------------------------------------------------------------------------------------------

// [
//   {
//     "address": "{\"address\": {\"city\": \"Builderland\", \"street\": \"456 Elm St\", \"zip\": \"67890\"}}",
//     "created_at": "2025-01-13 17:32:52",
//     "email": "bob@example.com",
//     "id": "1",
//     "name": "Bob"
//   },
//   {
//     "address": "{\"address\": {\"city\": \"Coderland\", \"street\": \"789 Oak St\", \"zip\": \"54321\"}}",
//     "created_at": "2025-01-13 17:32:52",
//     "email": "charlie@example.com",
//     "id": "2",
//     "name": "Charlie"
//   }
// ]

// sql data type where data can be in json  so ask sir to see if data will be there in json or  simple string ?

// -------------------------------------------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------------------------------------------

// use dotenv::dotenv;
// use sqlx::{MySqlPool, Row};
// use std::env;
// use datalayerapi_tidb::utils::sqlx::{check_connection, get_or_init_connection_pool};

// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
//     dotenv().ok();

//     // Get the database URL from the environment
//     // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

//     // Connect to the database
//     // let pool = MySqlPool::connect(&database_url).await?;

//    let sql_pool =  get_or_init_connection_pool("test").await;

//     // Create a sample table
//     sqlx::query(
//         r#"
//         CREATE TABLE IF NOT EXISTS users (
//             id INT AUTO_INCREMENT PRIMARY KEY,
//             name VARCHAR(100) NOT NULL,
//             email VARCHAR(100) NOT NULL UNIQUE
//         )
//         "#,
//     )
//     .execute(&sql_pool)
//     .await?;

//     // // // Insert sample data
//     sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
//         .bind("John Doe")
//         .bind("john.doe@example.com")
//         .execute(&sql_pool)
//         .await?;

//     // Fetch all databases
//     let rows = sqlx::query("SHOW DATABASES").fetch_all(&sql_pool).await?;

//     // Extract and print database names
//     println!("Databases:");
//     for row in rows {
//         let database_name: String = row.get("Database"); // Use the column name directly
//         println!("- {}", database_name);
//     }

//     // Fetch and print data
//     let rows = sqlx::query("SELECT id, name, email FROM users")
//         .fetch_all(&sql_pool)
//         .await?;

//     for row in rows {
//         let id: i32 = row.get("id");
//         let name: String = row.get("name");
//         let email: String = row.get("email");
//         println!("ID: {}, Name: {}, Email: {}", id, name, email);
//     }

//     Ok(())
// }
