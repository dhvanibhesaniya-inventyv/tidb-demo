use datalayerapi_tidb::utils::{self, logger::startLogger};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    startLogger();

    println!("Starting MySQLasync/TiDB connection test...");

    // Test database name
    let db_name = "test".to_string();

    // Test database connection
    println!("\nTesting database connection...");
    match utils::tidb::check_connection(db_name.clone()).await {
        Ok(true) => println!("✓ Connection successful!"),
        Ok(false) => println!("✗ Connection failed"),
        Err(e) => println!("✗ Connection error: {}", e),
    }

    // Create a test table
    println!("\nCreating test table...");
    let create_table_query = "
        CREATE TABLE IF NOT EXISTS test_users (
            id INT PRIMARY KEY AUTO_INCREMENT,
            name VARCHAR(100),
            email VARCHAR(100),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            address JSON,
            is_active BOOL


        )
    "
    .to_string();

    match utils::tidb::execute_query(create_table_query, db_name.clone()).await {
        Ok((status, data)) => println!("✓ Table creation: {} and data : {}", status, data),
        Err(e) => println!("✗ Table creation failed: {}", e),
    }

    // Insert test data
    println!("\nInserting test data...");
    let insert_queries = vec![
        "INSERT INTO test_users (name, email, address,is_active) VALUES ('Alice', 'alice@example.com', '{\"address\": {\"street\": \"123 Main St\", \"city\": \"Wonderland\", \"zip\": \"12345\"}}','true')",
        "INSERT INTO test_users (name, email, address,is_active) VALUES ('Bob', 'bob@example.com', '{\"address\": {\"street\": \"456 Elm St\", \"city\": \"Builderland\", \"zip\": \"67890\"}}','false')",
        "INSERT INTO test_users (name, email, address,is_active) VALUES ('Charlie', 'charlie@example.com', '{\"address\": {\"street\": \"789 Oak St\", \"city\": \"Coderland\", \"zip\": \"54321\"}}','true')",
    ];

    for query in insert_queries {
        match utils::tidb::execute_query(query.to_string(), db_name.clone()).await {
            Ok((status, data)) => println!("✓ Insert: {} and data : {}", status, data),
            Err(e) => println!("✗ Insert failed: {}", e),
        }
    }

    // Select and display data
    println!("\nRetrieving inserted data...");
    match utils::tidb::execute_query("SELECT * FROM test_users".to_string(), db_name.clone()).await
    {
        Ok((status, data)) => {
            println!("✓ Select: {}", status);
            println!("Data retrieved:");
            let parsed: serde_json::Value = serde_json::from_str(&data)?;
            println!("{}", serde_json::to_string_pretty(&parsed)?);
        }
        Err(e) => println!("✗ Select failed: {}", e),
    }

    // Test update functionality
    println!("\nTesting update functionality...");
    match utils::tidb::execute_query(
        "UPDATE test_users SET email = 'alice.new@example.com' WHERE name = 'Alice'".to_string(),
        db_name.clone(),
    )
    .await
    {
        Ok((status, data)) => println!("✓ Update: {} and data : {}", status, data),
        Err(e) => println!("✗ Update failed: {}", e),
    }

    // Verify update
    println!("\nVerifying update...");
    match utils::tidb::execute_query(
        "SELECT * FROM test_users WHERE name = 'Alice'".to_string(),
        db_name.clone(),
    )
    .await
    {
        Ok((status, data)) => {
            println!("✓ Select after update: {}", status);
            let parsed: serde_json::Value = serde_json::from_str(&data)?;
            println!("{}", serde_json::to_string_pretty(&parsed)?);
        }
        Err(e) => println!("✗ Select failed: {}", e),
    }

    // Clean up (optional - commented out to preserve data for inspection)

    println!("\nCleaning up test table...");
    match utils::tidb::execute_query("DROP TABLE test_users".to_string(), db_name.clone()).await {
        Ok((status, data)) => println!("✓ Cleanup: {} and data : {}", status, data),
        Err(e) => println!("✗ Cleanup failed: {}", e),
    }

    println!("\nTest completed!");
    Ok(())
}
