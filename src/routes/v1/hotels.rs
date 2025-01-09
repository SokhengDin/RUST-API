use rocket::{get, post, put, delete, serde::json::Json, State};
use utoipa::OpenApi;
use uuid::Uuid;
use crate::{schemas::hotels::*, services::hotels::HotelService};


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
pub async fn list_hotels(service: &State<HotelService>) -> Json<Vec<HotelSchemaOut>> {
    Json(service.list_hotels().await.unwrap())
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
pub async fn get_hotel(service: &State<HotelService>, id: &str) -> Option<Json<HotelSchemaOut>> {
    let uuid = Uuid::parse_str(id).ok()?;
    service.get_hotel(uuid).await.unwrap().map(Json)
}

/// Create a new hotel
#[utoipa::path(
    post
    , path = "/hotels"
    , tag  = "hotels"
    , request_body = HotelSchemaIn
    , responses(
        (status = 201, description = "Hotel created successfully", body = HotelSchemaOut),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/hotels", data = "<hotel>")]
pub async fn create_hotel(
    service: &State<HotelService>,
    hotel: Json<HotelSchemaIn>,
) -> Json<HotelSchemaOut> {

    Json(service.create_hotel(hotel.0).await.unwrap())
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
    service: &State<HotelService>,
    id: &str,
    hotel: Json<HotelSchemaIn>,
) -> Option<Json<HotelSchemaOut>> {

    let uuid = Uuid::parse_str(id).ok()?;

    service.update_hotel(uuid, hotel.0).await.unwrap().map(Json)
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
    service: &State<HotelService>, 
    id: &str
) -> Json<bool> {  
    let uuid = match Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Json(false),
    };
    
    Json(service.delete_hotel(uuid).await.unwrap())
}