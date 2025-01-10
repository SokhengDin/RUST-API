use rocket::{get, post, put, delete, serde::json::Json};
use uuid::Uuid;
use crate::{
    schemas::hotels::*,
    services::guards::ServiceGuard,
    services::traits::HotelServiceTrait,
};

/// List all hotels
#[utoipa::path(
    get
    , path  = "/hotels"
    , tag   = "hotels"
    , responses(
        (status = 200, description = "List of all hotels", body = Vec<HotelSchemaOut>)
    )
)]
#[get("/hotels")]
pub async fn list_hotels(
    guard: ServiceGuard
) -> Json<Vec<HotelSchemaOut>> {
    Json(guard.hotels().list_hotels().await.unwrap())
}

/// Get a specific hotel by ID
#[utoipa::path(
    get
    , path = "/hotels/{id}"
    , tag  = "hotels"
    , params(
        ("id" = String, Path, description = "Hotel UUID")
    )
    , responses(
        (status     = 200, description   = "Hotel found", body = HotelSchemaOut)
        , (status   = 404, description = "Hotel not found")
    )
)]
#[get("/hotels/<id>")]
pub async fn get_hotel(
    guard: ServiceGuard
    , id: &str
) -> Option<Json<HotelSchemaOut>> {
    let uuid = Uuid::parse_str(id).ok()?;
    guard.hotels().get_hotel(uuid).await.unwrap().map(Json)
}

/// Create a new hotel
#[utoipa::path(
    post
    , path = "/hotels"
    , tag  = "hotels"
    , request_body = HotelSchemaIn
    , responses(
        (status = 201, description = "Hotel created successfully", body = HotelSchemaOut)
        , (status = 400, description = "Invalid input")
    )
)]
#[post("/hotels", data = "<hotel>")]
pub async fn create_hotel(
    guard: ServiceGuard
    , hotel: Json<HotelSchemaIn>
) -> Json<HotelSchemaOut> {
    Json(guard.hotels().create_hotel(hotel.0).await.unwrap())
}

/// Update an existing hotel
#[utoipa::path(
    put
    , path = "/hotels/{id}"
    , tag  = "hotels"
    , params(
        ("id" = String, Path, description = "Hotel UUID")
    )
    , request_body  = HotelSchemaIn
    , responses(
        (status     = 200, description = "Hotel updated successfully", body = HotelSchemaOut)
        , (status   = 404, description = "Hotel not found")
    )
)]
#[put("/hotels/<id>", data = "<hotel>")]
pub async fn update_hotel(
    guard: ServiceGuard
    , id: &str
    , hotel: Json<HotelSchemaIn>
) -> Option<Json<HotelSchemaOut>> {
    let uuid = Uuid::parse_str(id).ok()?;
    guard.hotels().update_hotel(uuid, hotel.0).await.unwrap().map(Json)
}

/// Delete a hotel
#[utoipa::path(
    delete
    , path = "/hotels/{id}"
    , tag  = "hotels"
    , params(
        ("id" = String, Path, description = "Hotel UUID")
    )
    , responses(
        (status = 200, description = "Hotel deleted successfully")
        , (status = 404, description = "Hotel not found")
    )
)]
#[delete("/hotels/<id>")]
pub async fn delete_hotel(
    guard: ServiceGuard
    , id: &str
) -> Json<bool> {
    let uuid = match Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Json(false),
    };
    
    Json(guard.hotels().delete_hotel(uuid).await.unwrap())
}