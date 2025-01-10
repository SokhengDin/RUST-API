use sea_orm::DatabaseConnection;
use crate::services::{
    hotels::HotelService
    , rooms::RoomService
};

pub mod guards;
pub mod traits;

pub mod hotels;
pub mod guests;
pub mod rooms;
pub mod bookings;