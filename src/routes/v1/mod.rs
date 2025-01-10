use rocket::Route;
use utoipa::OpenApi;

pub mod hotels;
pub mod rooms;
pub mod bookings;
pub mod guests;

pub fn routes() -> Vec<Route> {
    routes![
        // Hotels endpoints
        hotels::list_hotels
        , hotels::get_hotel
        , hotels::create_hotel
        , hotels::update_hotel
        , hotels::delete_hotel

        // Rooms endpoints
        , rooms::list_rooms
        , rooms::get_room
        , rooms::create_room
        , rooms::update_room
        , rooms::delete_room
        , rooms::get_hotel_rooms
    ]
}

// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        // Hotels paths
        hotels::list_hotels
        , hotels::get_hotel
        , hotels::create_hotel
        , hotels::update_hotel
        , hotels::delete_hotel

        // Rooms paths
        , rooms::list_rooms
        , rooms::get_room
        , rooms::create_room
        , rooms::update_room
        , rooms::delete_room
        , rooms::get_hotel_rooms
    ),
    components(
        schemas(
            // Hotels schemas
            crate::schemas::hotels::HotelSchemaIn
            , crate::schemas::hotels::HotelSchemaOut

            // Rooms schemas
            , crate::schemas::rooms::RoomSchemaIn
            , crate::schemas::rooms::RoomSchemaOut
        )
    ),
    tags(
        (name = "hotels", description = "Hotel management endpoints")
        , (name = "rooms", description = "Room management endpoints")
    ),
    servers(
        (url = "/api/v1", description = "Version 1") 
    )
)]
pub struct ApiDoc;