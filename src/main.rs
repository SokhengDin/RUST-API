// Publish sub module
#[macro_use] extern crate rocket;
use std::sync::Arc;

use error::ErrorResponse;
use rocket::serde::json::Json;
use routes::v1::routes;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::v1::ApiDoc;

pub mod config;
pub mod models;
pub mod schemas;
pub mod services;
pub mod routes;
pub mod error;

#[catch(500)]
fn internal_error() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        code        : 500
        , message   : "Internal server error".to_string()
    })
}

#[catch(404)]
fn not_found() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        code        : 404
        , message   : "Resource not found".to_string()
    })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config  = config::AppConfig::new();
    let db = Arc::new(config.establish_connection().await);

    println!("📚 Configuration loaded");
    println!("✅ Database connected successfully");
    
    println!("🚀 Starting server on port {}", config.port);

    let _rocket: rocket::Rocket<rocket::Ignite> = rocket::build()
        .manage(db)
        .mount(
            "/api/v1"
            , routes::v1::routes()
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")  
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .register("/", catchers![internal_error, not_found])
        .launch()
        .await?;

    Ok(())
}