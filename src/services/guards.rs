use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use crate::services::{
    rooms::RoomService
    , hotels::HotelService
    , traits::{RoomServiceTrait, HotelServiceTrait}
};


pub struct ServiceGuard {
    db: Arc<DatabaseConnection>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ServiceGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = request.rocket().state::<Arc<DatabaseConnection>>()
            .expect("database connection not managed");
        Outcome::Success(ServiceGuard { db: db.clone() })
    }
}

impl ServiceGuard {
    pub fn rooms(&self) -> impl RoomServiceTrait + '_ {
        RoomService::new((*self.db).clone())
    }

    pub fn hotels(&self) -> impl HotelServiceTrait + '_ {
        HotelService::new((*self.db).clone())
    }
 }
