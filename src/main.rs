// Publish sub module
#[macro_use] extern crate rocket;
use rocket::http::hyper::Error;
use services::hotels::HotelService;

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

    println!("🚀 Starting server on port {}", config.port);

    let _rocket = rocket::build()
        .manage(db)
        .mount(
            "/"
            , routes![
                routes::hotels::list_hotels,
                routes::hotels::get_hotel,
                routes::hotels::create_hotel,
                routes::hotels::update_hotel,
                routes::hotels::delete_hotel,
            ]
        )
        .launch()
        .await?;

    Ok(())
}