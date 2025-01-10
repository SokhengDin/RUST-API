use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RoomSchemaIn {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub hotel_id: Uuid,

    #[schema(example = "101")]
    pub room_number: String,

    #[schema(example = "Deluxe")]
    pub room_type: String,

    #[schema(value_type = f64, example = 199.99)]
    pub price_per_night: Decimal,

    #[schema(example = true)]
    pub is_available: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RoomSchemaOut {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub hotel_id: Uuid,

    #[schema(example = "101")]
    pub room_number: String,

    #[schema(example = "Deluxe")]
    pub room_type: String,

    #[schema(value_type = f64, example = 199.99)]
    pub price_per_night: Decimal,

    #[schema(example = true)]
    pub is_available: bool,

    #[schema(example = "2024-01-10T12:00:00+00:00")]
    pub created_at: DateTime<FixedOffset>,

    #[schema(example = "2024-01-10T15:30:00+00:00")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}