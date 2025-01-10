// schemas/booking.rs
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};
use crate::models::sea_orm_active_enums::BookingStatus;
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct BookingSchemaIn {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub room_id         : Uuid
    
    , #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
      pub guest_id      : Uuid
    
    , #[schema(example = "2024-01-10T14:00:00+00:00")]
      pub check_in_date : DateTime<FixedOffset>
    
    , #[schema(example = "2024-01-15T11:00:00+00:00")]
      pub check_out_date: DateTime<FixedOffset>
    
    , #[schema(value_type = f64, example = 199.99)]
      pub total_price   : Decimal
    
    , #[schema(example = "pending")]
      pub status        : BookingStatus
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BookingSchemaOut {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id              : Uuid
    
    , #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
      pub room_id       : Uuid
    
    , #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
      pub guest_id      : Uuid
    
    , #[schema(example = "2024-01-10T14:00:00+00:00")]
      pub check_in_date : DateTime<FixedOffset>
    
    , #[schema(example = "2024-01-15T11:00:00+00:00")]
      pub check_out_date: DateTime<FixedOffset>
    
    , #[schema(value_type = f64, example = 199.99)]
      pub total_price   : Decimal
    
    , #[schema(example = "confirmed")]
      pub status        : BookingStatus
    
    , #[schema(example = "2024-01-10T12:00:00+00:00")]
      pub created_at    : DateTime<FixedOffset>
    
    , #[schema(example = "2024-01-10T15:30:00+00:00")]
      pub updated_at    : Option<DateTime<FixedOffset>>
}