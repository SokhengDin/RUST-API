use rocket::{get, post, put, delete, serde::json::Json};
use uuid::Uuid;
use crate::{
    schemas::rooms::*,
    services::guards::ServiceGuard,
    services::traits::RoomServiceTrait,  
};

use crate::error::ApiError;

/// List all rooms
#[utoipa::path(
    get
    , path  = "/rooms"
    , tag   = "rooms"
    , responses(
        (status = 200, description = "List of all rooms", body = Vec<RoomSchemaOut>)
    )
)]
#[get("/rooms")]
pub async fn list_rooms(
    guard: ServiceGuard 
) -> Json<Vec<RoomSchemaOut>> {
    Json(guard.rooms().get_all_rooms().await.unwrap())
}

/// Get a specific room
#[utoipa::path(
    get
    , path  = "/rooms/{id}"
    , tag   = "rooms"
    , params(
        ("id" = String, Path, description = "Room UUID")
    )
    , responses(
        (status = 200, description = "Room found", body = RoomSchemaOut)
        , (status = 404, description = "Room not found")
    )
)]
#[get("/rooms/<id>")]
pub async fn get_room(
    guard: ServiceGuard  
    , id: &str
) -> Option<Json<RoomSchemaOut>> {
    let uuid = Uuid::parse_str(id).ok()?;
    guard.rooms().get_room(uuid).await.unwrap().map(Json)
}

/// Create a new room
#[utoipa::path(
    post
    , path = "/rooms"
    , tag  = "rooms"
    , request_body = RoomSchemaIn
    , responses(
        (status = 201, description = "Room created successfully", body = RoomSchemaOut)
        , (status = 400, description = "Invalid input")
    )
)]
#[post("/rooms", data = "<room>")]
pub async fn create_room(
    guard: ServiceGuard  
    , room: Json<RoomSchemaIn>
) -> Json<RoomSchemaOut> {
    Json(guard.rooms().create_room(room.0).await.unwrap())
}

/// Update a room
#[utoipa::path(
    put
    , path = "/rooms/{id}"
    , tag  = "rooms"
    , params(
        ("id" = String, Path, description = "Room UUID")
    )
    , request_body = RoomSchemaIn
    , responses(
        (status = 200, description = "Room updated successfully", body = RoomSchemaOut)
        , (status = 404, description = "Room not found")
    )
)]
#[put("/rooms/<id>", data = "<room>")]
pub async fn update_room(
    guard: ServiceGuard,
    id: &str,
    room: Json<RoomSchemaIn>,
) -> Result<Option<Json<RoomSchemaOut>>, ApiError> {
    let uuid = Uuid::parse_str(id)
        .map_err(|_| ApiError::RoomNotFound(id.to_string()))?;

    let updated_room = guard.rooms().update_room(uuid, room.0).await?;
    Ok(updated_room.map(Json))
}

/// Delete a room
#[utoipa::path(
    delete
    , path = "/rooms/{id}"
    , tag  = "rooms"
    , params(
        ("id" = String, Path, description = "Room UUID")
    )
    , responses(
        (status = 200, description = "Room deleted successfully")
        , (status = 404, description = "Room not found")
    )
)]
#[delete("/rooms/<id>")]
pub async fn delete_room(
    guard: ServiceGuard
    , id: &str
) -> Json<bool> {
    let uuid = match Uuid::parse_str(id) {
        Ok(id) => id,
        Err(_) => return Json(false),
    };
    
    Json(guard.rooms().delete_room(uuid).await.unwrap())
}

/// Get rooms by hotel
#[utoipa::path(
    get
    , path = "/hotels/{hotel_id}/rooms"
    , tag  = "rooms"
    , params(
        ("hotel_id" = String, Path, description = "Hotel UUID")
    )
    , responses(
        (status = 200, description = "List of rooms for the hotel", body = Vec<RoomSchemaOut>)
        , (status = 404, description = "Hotel not found")
    )
)]
#[get("/hotels/<hotel_id>/rooms")]
pub async fn get_hotel_rooms(
    guard: ServiceGuard
    , hotel_id: &str
) -> Option<Json<Vec<RoomSchemaOut>>> {
    let uuid = Uuid::parse_str(hotel_id).ok()?;
    Some(Json(guard.rooms().get_rooms_by_hotel(uuid).await.unwrap()))
}