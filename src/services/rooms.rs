use sea_orm::*;
use uuid::Uuid;
use chrono::{Utc, FixedOffset};
use crate::{
    models::rooms
    , schemas::rooms::*
    , services::traits::RoomServiceTrait
    , error::ApiError
};

#[derive(Clone)]
pub struct RoomService {
    db: DatabaseConnection
}

impl RoomService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    async fn check_hotel_exists(&self, hotel_id: Uuid) -> Result<bool, ApiError> {
        use crate::models::hotels::Entity as Hotel;
        Ok(Hotel::find_by_id(hotel_id)
            .one(&self.db)
            .await
            .map_err(ApiError::Database)?
            .is_some())
    }
}

#[async_trait]
impl RoomServiceTrait for RoomService {
    async fn get_all_rooms(&self) -> Result<Vec<RoomSchemaOut>, ApiError> {
        let res = rooms::Entity::find()
            .all(&self.db)
            .await
            .map_err(ApiError::Database)?;

        Ok(res.into_iter().map(|h| RoomSchemaOut {
            id              : h.id
            , hotel_id      : h.hotel_id
            , room_number   : h.room_number
            , room_type     : h.room_type
            , price_per_night   : h.price_per_night
            , is_available      : h.is_available
            , created_at        : h.created_at
            , updated_at        : h.updated_at
        }).collect())
    }

    async fn get_room(&self, id: Uuid) -> Result<Option<RoomSchemaOut>, ApiError> {
        let res = rooms::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(ApiError::Database)?;

        Ok(res.map(|h| RoomSchemaOut {
            id              : h.id
            , hotel_id      : h.hotel_id
            , room_number   : h.room_number
            , room_type     : h.room_type
            , price_per_night   : h.price_per_night
            , is_available      : h.is_available
            , created_at        : h.created_at
            , updated_at        : h.updated_at
        }))
    }

    async fn create_room(&self, req: RoomSchemaIn) -> Result<RoomSchemaOut, ApiError> {
        // Check if hotel exists first
        if !self.check_hotel_exists(req.hotel_id).await? {
            return Err(ApiError::HotelNotFound(req.hotel_id.to_string()));
        }

        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());

        let room = rooms::ActiveModel {
            id              : Set(Uuid::new_v4())
            , hotel_id      : Set(req.hotel_id)
            , room_number   : Set(req.room_number)
            , room_type     : Set(req.room_type)
            , price_per_night   : Set(req.price_per_night)
            , is_available      : Set(req.is_available)
            , created_at        : Set(now)
            , updated_at        : Set(None)
        };

        let res = room.insert(&self.db)
            .await
            .map_err(ApiError::Database)?;

        Ok(RoomSchemaOut {
            id              : res.id
            , hotel_id      : res.hotel_id
            , room_number   : res.room_number
            , room_type     : res.room_type
            , price_per_night   : res.price_per_night
            , is_available      : res.is_available
            , created_at        : res.created_at
            , updated_at        : res.updated_at
        })
    }

    async fn update_room(&self, id: Uuid, req: RoomSchemaIn) -> Result<Option<RoomSchemaOut>, ApiError> {
        // Check if hotel exists
        if !self.check_hotel_exists(req.hotel_id).await? {
            return Err(ApiError::HotelNotFound(req.hotel_id.to_string()));
        }

        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());

        let room = match rooms::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(ApiError::Database)? {
                Some(h) => h,
                None => return Ok(None),
            };
        
        let mut room: rooms::ActiveModel = room.into();

        room.hotel_id           = Set(req.hotel_id);
        room.room_number        = Set(req.room_number);
        room.room_type          = Set(req.room_type);
        room.price_per_night    = Set(req.price_per_night);
        room.is_available       = Set(req.is_available);
        room.updated_at         = Set(Some(now));

        let updated = room.update(&self.db)
            .await
            .map_err(ApiError::Database)?;

        Ok(Some(RoomSchemaOut {
            id              : updated.id
            , hotel_id      : updated.hotel_id
            , room_number   : updated.room_number
            , room_type     : updated.room_type
            , price_per_night   : updated.price_per_night
            , is_available      : updated.is_available
            , created_at        : updated.created_at
            , updated_at        : updated.updated_at
        }))
    }

    async fn delete_room(&self, id: Uuid) -> Result<bool, ApiError> {
        let res = rooms::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(ApiError::Database)?;

        Ok(res.rows_affected > 0)
    }

    async fn get_rooms_by_hotel(&self, hotel_id: Uuid) -> Result<Vec<RoomSchemaOut>, ApiError> {
        // Check if hotel exists first
        if !self.check_hotel_exists(hotel_id).await? {
            return Err(ApiError::HotelNotFound(hotel_id.to_string()));
        }

        let res = rooms::Entity::find()
            .filter(rooms::Column::HotelId.eq(hotel_id))
            .all(&self.db)
            .await
            .map_err(ApiError::Database)?;

        Ok(res.into_iter().map(|h| RoomSchemaOut {
            id              : h.id
            , hotel_id      : h.hotel_id
            , room_number   : h.room_number
            , room_type     : h.room_type
            , price_per_night   : h.price_per_night
            , is_available      : h.is_available
            , created_at        : h.created_at
            , updated_at        : h.updated_at
        }).collect())
    }
}