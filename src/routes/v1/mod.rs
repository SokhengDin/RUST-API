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

        // Guests endpoints
        , guests::list_guests
        , guests::get_guest
        , guests::create_guest
        , guests::update_guest
        , guests::delete_guest

        // Booking paths
        , bookings::list_bookings
        , bookings::get_booking
        , bookings::create_booking
        , bookings::update_booking
        , bookings::delete_booking
        , bookings::get_guest_bookings
        , bookings::get_room_bookings
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

        // Guest paths
        , guests::list_guests
        , guests::get_guest
        , guests::create_guest
        , guests::update_guest
        , guests::delete_guest

        // Bookings endpoints
        , bookings::list_bookings
        , bookings::get_booking
        , bookings::create_booking
        , bookings::update_booking
        , bookings::delete_booking
        , bookings::get_guest_bookings
        , bookings::get_room_bookings
    ),
    components(
        schemas(
            // Hotels schemas
            crate::schemas::hotels::HotelSchemaIn
            , crate::schemas::hotels::HotelSchemaOut

            // Rooms schemas
            , crate::schemas::rooms::RoomSchemaIn
            , crate::schemas::rooms::RoomSchemaOut

            // Guests schemas
            , crate::schemas::guests::GuestSchemaIn
            , crate::schemas::guests::GuestSchemaOut

            // Bookings schemas
            , crate::schemas::booking::BookingSchemaIn
            , crate::schemas::booking::BookingSchemaOut
            , crate::models::sea_orm_active_enums::BookingStatus
        )
    ),
    tags(
        (name = "hotels", description = "Hotel management endpoints")
        , (name = "rooms", description = "Room management endpoints")
        , (name = "guests", description = "Guest management endpoints")
        , (name = "bookings", description = "Booking management endpoints")
    ),
    servers(
        (url = "/api/v1", description = "Version 1") 
    )
)]
pub struct ApiDoc;