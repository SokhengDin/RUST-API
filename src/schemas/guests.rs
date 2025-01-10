use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GuestSchemaIn {
    #[schema(example = "John")]
    pub first_name      : String
    
    , #[schema(example = "Doe")]
      pub last_name     : String
    
    , #[schema(example = "john.doe@example.com")]
      pub email         : String
    
    , #[schema(example = "+1234567890")]
      pub phone         : Option<String>
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "phone": "+1234567890",
    "created_at": "2024-01-10T12:00:00+00:00",
    "updated_at": "2024-01-10T12:00:00+00:00"
}))]
pub struct GuestSchemaOut {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id              : Uuid
    
    , #[schema(example = "John")]
      pub first_name    : String
    
    , #[schema(example = "Doe")]
      pub last_name     : String
    
    , #[schema(example = "john.doe@example.com")]
      pub email         : String
    
    , #[schema(example = "+1234567890")]
      pub phone         : String
    
    , #[schema(example = "2024-01-10T12:00:00+00:00")]
      pub created_at    : DateTime<FixedOffset>
    
    , pub updated_at    : Option<DateTime<FixedOffset>>
}