use rocket::{get, post, put, delete, serde::json::Json};
use uuid::Uuid;
use crate::{
    schemas::booking::*,
    services::guards::ServiceGuard,
    services::traits::BookingServiceTrait,
};

/// List all bookings
#[utoipa::path(
    get
    , path  = "/bookings"
    , tag   = "bookings"
    , responses(
        (status = 200, description = "List of all bookings", body = Vec<BookingSchemaOut>)
    )
)]
#[get("/bookings")]
pub async fn list_bookings(
    guard: ServiceGuard
) -> Json<Vec<BookingSchemaOut>> {
    Json(guard.bookings().list_bookings().await.unwrap())
}

/// Get a specific booking by ID
#[utoipa::path(
    get
    , path  = "/bookings/{id}"
    , tag   = "bookings"
    , params(
        ("id" = String, Path, description = "Booking UUID")
    )
    , responses(
        (status     = 200, description = "Booking found", body = BookingSchemaOut)
        , (status   = 404, description = "Booking not found")
    )
)]
#[get("/bookings/<id>")]
pub async fn get_booking(
    guard   : ServiceGuard
    , id    : &str
) -> Option<Json<BookingSchemaOut>> {
    let uuid = Uuid::parse_str(id).ok()?;
    guard.bookings().get_booking(uuid).await.unwrap().map(Json)
}

/// Create a new booking
#[utoipa::path(
    post
    , path  = "/bookings"
    , tag   = "bookings"
    , request_body  = BookingSchemaIn
    , responses(
        (status     = 201, description = "Booking created successfully", body = BookingSchemaOut)
        , (status   = 400, description = "Invalid input")
    )
)]
#[post("/bookings", data = "<booking>")]
pub async fn create_booking(
    guard       : ServiceGuard
    , booking   : Json<BookingSchemaIn>
) -> Json<BookingSchemaOut> {
    Json(guard.bookings().create_booking(booking.0).await.unwrap())
}

/// Update an existing booking
#[utoipa::path(
    put
    , path  = "/bookings/{id}"
    , tag   = "bookings"
    , params(
        ("id" = String, Path, description = "Booking UUID")
    )
    , request_body  = BookingSchemaIn
    , responses(
        (status     = 200, description = "Booking updated successfully", body = BookingSchemaOut)
        , (status   = 404, description = "Booking not found")
    )
)]
#[put("/bookings/<id>", data = "<booking>")]
pub async fn update_booking(
    guard       : ServiceGuard
    , id        : &str
    , booking   : Json<BookingSchemaIn>
) -> Option<Json<BookingSchemaOut>> {
    let uuid = Uuid::parse_str(id).ok()?;
    guard.bookings().update_booking(uuid, booking.0).await.unwrap().map(Json)
}

/// Delete a booking
#[utoipa::path(
    delete
    , path  = "/bookings/{id}"
    , tag   = "bookings"
    , params(
        ("id" = String, Path, description = "Booking UUID")
    )
    , responses(
        (status     = 200, description = "Booking deleted successfully")
        , (status   = 404, description = "Booking not found")
    )
)]
#[delete("/bookings/<id>")]
pub async fn delete_booking(
    guard   : ServiceGuard
    , id    : &str
) -> Json<bool> {
    let uuid = match Uuid::parse_str(id) {
        Ok(id)  => id,
        Err(_)  => return Json(false),
    };
    
    Json(guard.bookings().delete_booking(uuid).await.unwrap())
}

/// Get bookings for a specific guest
#[utoipa::path(
    get
    , path  = "/guests/{guest_id}/bookings"
    , tag   = "bookings"
    , params(
        ("guest_id" = String, Path, description = "Guest UUID")
    )
    , responses(
        (status = 200, description = "List of guest's bookings", body = Vec<BookingSchemaOut>)
    )
)]
#[get("/guests/<guest_id>/bookings")]
pub async fn get_guest_bookings(
    guard       : ServiceGuard
    , guest_id  : &str
) -> Json<Vec<BookingSchemaOut>> {
    let uuid = Uuid::parse_str(guest_id).unwrap_or_default();
    Json(guard.bookings().get_guest_bookings(uuid).await.unwrap())
}

/// Get bookings for a specific room
#[utoipa::path(
    get
    , path  = "/rooms/{room_id}/bookings"
    , tag   = "bookings"
    , params(
        ("room_id" = String, Path, description = "Room UUID")
    )
    , responses(
        (status = 200, description = "List of room's bookings", body = Vec<BookingSchemaOut>)
    )
)]
#[get("/rooms/<room_id>/bookings")]
pub async fn get_room_bookings(
    guard       : ServiceGuard
    , room_id   : &str
) -> Json<Vec<BookingSchemaOut>> {
    let uuid = Uuid::parse_str(room_id).unwrap_or_default();
    Json(guard.bookings().get_room_bookings(uuid).await.unwrap())
}