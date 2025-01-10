// routes/v1/guests.rs
use rocket::{get, post, put, delete, serde::json::Json};
use uuid::Uuid;
use crate::{
    schemas::guests::*,
    services::guards::ServiceGuard,
    services::traits::GuestServiceTrait,
};

/// List all guests
#[utoipa::path(
    get
    , path  = "/guests"
    , tag   = "guests"
    , responses(
        (status = 200, description = "List of all guests", body = Vec<GuestSchemaOut>)
    )
)]
#[get("/guests")]
pub async fn list_guests(
    guard: ServiceGuard
) -> Json<Vec<GuestSchemaOut>> {
    Json(guard.guests().list_guests().await.unwrap())
}

/// Get a specific guest by ID
#[utoipa::path(
    get
    , path  = "/guests/{id}"
    , tag   = "guests"
    , params(
        ("id" = String, Path, description = "Guest UUID")
    )
    , responses(
        (status     = 200, description = "Guest found", body = GuestSchemaOut)
        , (status   = 404, description = "Guest not found")
    )
)]
#[get("/guests/<id>")]
pub async fn get_guest(
    guard   : ServiceGuard
    , id    : &str
) -> Option<Json<GuestSchemaOut>> {
    let uuid = Uuid::parse_str(id).ok()?;
    guard.guests().get_guest(uuid).await.unwrap().map(Json)
}

/// Create a new guest
#[utoipa::path(
    post
    , path  = "/guests"
    , tag   = "guests"
    , request_body  = GuestSchemaIn
    , responses(
        (status     = 201, description = "Guest created successfully", body = GuestSchemaOut)
        , (status   = 400, description = "Invalid input")
    )
)]
#[post("/guests", data = "<guest>")]
pub async fn create_guest(
    guard       : ServiceGuard
    , guest     : Json<GuestSchemaIn>
) -> Json<GuestSchemaOut> {
    Json(guard.guests().create_guest(guest.0).await.unwrap())
}

/// Update an existing guest
#[utoipa::path(
    put
    , path  = "/guests/{id}"
    , tag   = "guests"
    , params(
        ("id" = String, Path, description = "Guest UUID")
    )
    , request_body  = GuestSchemaIn
    , responses(
        (status     = 200, description = "Guest updated successfully", body = GuestSchemaOut)
        , (status   = 404, description = "Guest not found")
    )
)]
#[put("/guests/<id>", data = "<guest>")]
pub async fn update_guest(
    guard       : ServiceGuard
    , id        : &str
    , guest     : Json<GuestSchemaIn>
) -> Option<Json<GuestSchemaOut>> {
    let uuid = Uuid::parse_str(id).ok()?;
    guard.guests().update_guest(uuid, guest.0).await.unwrap().map(Json)
}

/// Delete a guest
#[utoipa::path(
    delete
    , path  = "/guests/{id}"
    , tag   = "guests"
    , params(
        ("id" = String, Path, description = "Guest UUID")
    )
    , responses(
        (status     = 200, description = "Guest deleted successfully")
        , (status   = 404, description = "Guest not found")
    )
)]
#[delete("/guests/<id>")]
pub async fn delete_guest(
    guard   : ServiceGuard
    , id    : &str
) -> Json<bool> {
    let uuid = match Uuid::parse_str(id) {
        Ok(id)  => id,
        Err(_)  => return Json(false),
    };
    
    Json(guard.guests().delete_guest(uuid).await.unwrap())
}