use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};


#[derive(Deserialize, Serialize, ToSchema)]
pub struct HotelSchemaIn {
    #[schema(example = "Grand Hotel")]
    pub name          : String
    ,
    #[schema(example = "123 Main Street")] 
    pub address       : String
    , 
    #[schema(example = 4.5)]
    pub rating        : f64
    ,
    #[schema(example = "Luxury hotel in city center")] 
    pub description   : Option<String>
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HotelSchemaOut {
    pub id          : Uuid
    , pub name      : String
    , pub address   : String
    , pub rating    : f64
    , pub description: Option<String>
    , pub created_at    : DateTime<FixedOffset>
    , pub updated_at    : Option<DateTime<FixedOffset>>
}

