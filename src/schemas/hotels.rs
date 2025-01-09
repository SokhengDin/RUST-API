use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};


#[derive(Debug, Serialize, Deserialize)]
pub struct HotelSchemaIn {
    pub name        : String
    , pub address   : String
    , pub rating    : f64
    , pub description   : Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotelSchemaOut {
    pub id          : Uuid
    , pub name      : String
    , pub address   : String
    , pub rating    : f64
    , pub description: Option<String>
    , pub created_at    : DateTime<FixedOffset>
    , pub updated_at    : Option<DateTime<FixedOffset>>
}

