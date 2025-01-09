use std::env;
use sea_orm::DatabaseConnection;

pub struct AppConfig {
    pub database_url    : String
    , pub port          : u16
}


impl AppConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        // Get db vars
        let db_user = env::var("DB_USER").expect("DB_USER must be set");
        let db_pass = env::var("DB_PASS").expect("DB_PASS must be set");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
        let db_port = env::var("DB_PORT").expect("DB_PORT must be set");

        // Construct database url
        let database_url    = format!(
             "postgres://{}:{}@localhost:{}/{}"
             , db_user, db_pass, db_port, db_name
        );

        println!("Checking database url {}", database_url);

        // Get server url

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("PORT must be a number");

        Self {
            database_url
            , port
        }
    }
    

    pub async fn establish_connection(&self) -> DatabaseConnection {
        sea_orm::Database::connect(&self.database_url)
            .await
            .expect("Faield to connect to database")
    }
}