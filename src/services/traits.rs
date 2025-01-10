use sea_orm::DbErr;
use sea_orm::prelude::async_trait;
use uuid::Uuid;
use crate::schemas::{rooms::*, hotels::*, guests::*, booking::*};
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


#[async_trait]
pub trait GuestServiceTrait {
    async fn list_guests(&self) -> Result<Vec<GuestSchemaOut>, DbErr>;
    async fn get_guest(&self, id: Uuid) -> Result<Option<GuestSchemaOut>, DbErr>;
    async fn create_guest(&self, guest: GuestSchemaIn) -> Result<GuestSchemaOut, DbErr>;
    async fn update_guest(&self, id: Uuid, guest: GuestSchemaIn) -> Result<Option<GuestSchemaOut>, DbErr>;
    async fn delete_guest(&self, id: Uuid) -> Result<bool, DbErr>;
}

#[async_trait]
pub trait BookingServiceTrait {
    async fn create_booking(
        &self
        , booking    : BookingSchemaIn
    ) -> Result<BookingSchemaOut, DbErr>;
    
    async fn get_booking(
        &self
        , id        : Uuid
    ) -> Result<Option<BookingSchemaOut>, DbErr>;
    
    async fn list_bookings(&self) -> Result<Vec<BookingSchemaOut>, DbErr>;
    
    async fn update_booking(
        &self
        , id        : Uuid
        , booking   : BookingSchemaIn
    ) -> Result<Option<BookingSchemaOut>, DbErr>;
    
    async fn delete_booking(
        &self
        , id        : Uuid
    ) -> Result<bool, DbErr>;
    
    async fn get_guest_bookings(
        &self
        , guest_id  : Uuid
    ) -> Result<Vec<BookingSchemaOut>, DbErr>;
    
    async fn get_room_bookings(
        &self
        , room_id   : Uuid
    ) -> Result<Vec<BookingSchemaOut>, DbErr>;
}