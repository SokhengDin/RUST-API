use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};


#[derive(Deserialize, Serialize, ToSchema)]
pub struct BookingSchemaIn {
    #[schema(example    = "Room ID")]
    pub room_id     : Uuid
    , #[schema(example  = "Guest ID")]
      pub guest_id  : Uuid
    , #[schema(example  = "Check in Date")]
      pub check_in_date     : DateTime<FixedOffset>
    , #[schema(example  = "Check out Date")]
      pub check_out_date    : DateTime<FixedOffset>
    , #[schema(example  = "Total Price")]
      pub total_price       : Decimal
    , #[schema(example  = "Booking Status")]
      pub status            : String
      
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct BookingSchemaOut {
    pub id          : Uuid
    , pub name      : String
    , pub address   : String
    , pub rating    : f64
    , pub description: Option<String>
    , pub created_at    : DateTime<FixedOffset>
    , pub updated_at    : Option<DateTime<FixedOffset>>
}

