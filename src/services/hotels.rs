use rocket::{form::name, http::hyper::Error};
use sea_orm::*;
use uuid::Uuid;
use chrono::{Utc, FixedOffset, DateTime};
use crate::{models::hotels, schemas::hotels::*};

pub struct HotelService {
    db  : DatabaseConnection
}

impl HotelService {
    fn new(db   : DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_hotel(
        &self
        , req   : HotelSchemaIn
    ) -> Result<HotelSchemaOut, DbErr> {

        // Datetime
        let now     = Utc::now().with_timezone(&FixedOffset::east(0));

        let hotel   = hotels::ActiveModel {
            id      : Set(Uuid::new_v4())
            , name      : Set(req.name)
            , address   : Set(req.address)
            , rating    : Set(req.rating)
            , description   : Set(req.description)
            , created_at    : Set(now)
            , updated_at    : Set(None)
        };

        let res = hotel.insert(&self.db).await?;
        Ok(HotelSchemaOut {
            id      : res.id
            , name  : res.name
            , address   : res.address
            , rating    : res.rating
            , description   : res.description
            , created_at    : res.created_at
            , updated_at    : res.updated_at 
        })

    }


    pub async fn get_hotel(
        &self
        ,   id  : Uuid
    ) -> Result<Option<HotelSchemaOut>, DbErr> {
        let res = hotels::Entity::find_by_id(id)
            .one(&self.db)
            .await?;

        Ok(res.map(|h| HotelSchemaOut {
            id      : h.id
            , name  : h.name
            , address   : h.address
            , rating    : h.rating
            , description   : h.description
            , created_at    : h.created_at
            , updated_at    : h.updated_at           
        }))
    }


    pub async fn list_hotels(&self) -> Result<Vec<HotelSchemaOut>, DbErr> {
        
        let res = hotels::Entity::find()
            .all(&self.db)
            .await?;

        Ok(res.into_iter().map(|h| HotelSchemaOut {
            id      : h.id
            , name  : h.name
            , address   : h.address
            , rating    : h.rating
            , description   : h.description
            , created_at    : h.created_at
            , updated_at    : h.updated_at    
        }).collect())
    }

    pub async fn update_hotel(
        &self
        , id  : Uuid
        , req : HotelSchemaIn
    ) -> Result<Option<HotelSchemaOut>, DbErr> {

        // Tiem
        let now     = Utc::now().with_timezone(&FixedOffset::east(0));
        let hotel   = match hotels::Entity::find_by_id(id).one(&self.db).await {
            Some(h) => h,
            None => return Ok(None),
        };

        let mut hotel : hotels::ActiveModel = hotel.into();

        if let name = req.name {
            hotel.name        = Set(name);
        }
        if let address = req.address {
            hotel.address     = Set(address);
        }
        if let rating = req.rating {
            hotel.rating      = Set(rating);
        }
        if req.description.is_some() {
            hotel.description = Set(req.description);
        }

        hotel.updated_at      = Set(Some(now));

        let updated: hotels::Model  = hotel.update(&self.db).await?;

        Ok(Some(HotelSchemaOut {
            id      : updated.id
            , name  : updated.name
            , address   : updated.address
            , rating    : updated.rating
            , description   : updated.description
            , created_at    : updated.created_at
            , updated_at    : updated.updated_at           
        }))
        


    }

    pub async fn delete_hotel(
        &self
        , id  : Uuid
    ) -> Result<bool, DbErr> {
        let res = hotels::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        Ok(res.rows_affected > 0)
    }


}