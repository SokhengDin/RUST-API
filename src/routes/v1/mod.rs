use rocket::Route;
use utoipa::OpenApi;

pub mod hotels;


pub fn routes() -> Vec<Route> {
    routes![
        hotels::list_hotels,
        hotels::get_hotel,
        hotels::create_hotel,
        hotels::update_hotel,
        hotels::delete_hotel,
    ]
}


// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        hotels::list_hotels,
        hotels::get_hotel,
        hotels::create_hotel,
        hotels::update_hotel,
        hotels::delete_hotel
    ),
    components(
        schemas(
            crate::schemas::hotels::HotelSchemaIn,
            crate::schemas::hotels::HotelSchemaOut
        )
    ),
    tags(
        (name = "hotels", description = "Hotel management endpoints")
    ),
    servers(
        (url = "/api/v1", description = "Version 1") 
    )
)]
pub struct ApiDoc;