// Publish sub module
#[macro_use] extern crate rocket;
use routes::v1::routes;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use services::hotels::HotelService;

use crate::routes::v1::ApiDoc;

pub mod config;
pub mod models;
pub mod schemas;
pub mod services;
pub mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config  = config::AppConfig::new();

    println!("📚 Configuration loaded");

    let db = config.establish_connection().await;
    println!("✅ Database connected successfully");
    
    // Service
    let service = HotelService::new(db);
    println!("✅ Services initialized");
    println!("🚀 Starting server on port {}", config.port);

    let _rocket = rocket::build()
        .manage(service)
        .mount(
            "/api/v1"
            , routes::v1::routes()
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")  
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .launch()
        .await?;

    Ok(())
}