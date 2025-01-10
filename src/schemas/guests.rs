use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};


#[derive(Deserialize, Serialize, ToSchema)]
pub struct GuestSchemaIn {
    #[schema(example = "First Name")]
    pub first_name  : String
    , #[schema(example = "Last Name")]
      pub last_name : String
    , #[schema(example = "Email Address")]
      pub email     : String
    , #[schema(example = "Phone Number") ]
      pub phone     : Option<String>
}


#[derive(Deserialize, Serialize, ToSchema)]
pub struct GuestSchemaOut {
    pub id              : Uuid
    , pub first_name    : String
    , pub last_name     : String
    , pub email         : String
    , pub phone         : String
    , pub created_at    : DateTime<FixedOffset>
    , pub updated_at    : Option<DateTime<FixedOffset>>
}

