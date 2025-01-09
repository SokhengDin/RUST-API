use rocket::{get, post, put, delete, serde::json::Json, State};
use uuid::Uuid;
use crate::{schemas::hotels::*, services::hotels::HotelService};

 #[get("/hotels")]
 pub async fn list_hotels(
    service: &State<HotelService>
 ) -> Json<Vec<HotelSchemaOut>> {

    Json(service.list_hotels().await.unwrap())
 }

 #[get("/hotels/<id>")]
 pub async fn get_hotel(service: &State<HotelService>, id: &str) -> Option<Json<HotelSchemaOut>> {
     let uuid = Uuid::parse_str(id).ok()?;
     service.get_hotel(uuid).await.unwrap().map(Json)
 }

 #[post("/hotels", data = "<hotel>")]
pub async fn create_hotel(
    service: &State<HotelService>,
    hotel: Json<HotelSchemaIn>,
) -> Json<HotelSchemaOut> {

    Json(service.create_hotel(hotel.0).await.unwrap())
}

#[put("/hotels/<id>", data = "<hotel>")]
pub async fn update_hotel(
    service: &State<HotelService>,
    id: &str,
    hotel: Json<HotelSchemaIn>,
) -> Option<Json<HotelSchemaOut>> {

    let uuid = Uuid::parse_str(id).ok()?;

    service.update_hotel(uuid, hotel.0).await.unwrap().map(Json)
}

#[delete("/hotels/<id>")]
pub async fn delete_hotel(service: &State<HotelService>, id: &str) -> Json<bool> {
    

    Json(service.delete_hotel(uuid).await.unwrap())
}