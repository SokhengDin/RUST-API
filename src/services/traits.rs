use sea_orm::DbErr;
use sea_orm::prelude::async_trait;
use uuid::Uuid;
use crate::schemas::{rooms::*, hotels::*};
use crate::error::ApiError;


#[async_trait]
pub trait HotelServiceTrait {
    async fn list_hotels(&self) -> Result<Vec<HotelSchemaOut>, DbErr>;
    async fn get_hotel(&self, id: Uuid) -> Result<Option<HotelSchemaOut>, DbErr>;
    async fn create_hotel(&self, hotel: HotelSchemaIn) -> Result<HotelSchemaOut, DbErr>;
    async fn update_hotel(&self, id: Uuid, hotel: HotelSchemaIn) -> Result<Option<HotelSchemaOut>, DbErr>;
    async fn delete_hotel(&self, id: Uuid) -> Result<bool, DbErr>;
}

#[async_trait]
pub trait RoomServiceTrait {
    async fn get_all_rooms(&self) -> Result<Vec<RoomSchemaOut>, ApiError>;
    async fn get_room(&self, id: Uuid) -> Result<Option<RoomSchemaOut>, ApiError>;
    async fn create_room(&self, room: RoomSchemaIn) -> Result<RoomSchemaOut, ApiError>;
    async fn update_room(&self, id: Uuid, room: RoomSchemaIn) -> Result<Option<RoomSchemaOut>, ApiError>;
    async fn delete_room(&self, id: Uuid) -> Result<bool, ApiError>;
    async fn get_rooms_by_hotel(&self, hotel_id: Uuid) -> Result<Vec<RoomSchemaOut>, ApiError>;
}