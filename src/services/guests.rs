// services/guests.rs
use sea_orm::*;
use uuid::Uuid;
use chrono::{Utc, FixedOffset};
use crate::{
    models::guests,
    schemas::guests::*,
    services::traits::GuestServiceTrait,
};

#[derive(Clone)]
pub struct GuestService {
    db  : DatabaseConnection
}

impl GuestService {
    pub fn new(db   : DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl GuestServiceTrait for GuestService {
    
    async fn create_guest(
        &self
        , req   : GuestSchemaIn
    ) -> Result<GuestSchemaOut, DbErr> {
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());

        let guest = guests::ActiveModel {
            id              : Set(Uuid::new_v4())
            , first_name    : Set(req.first_name)
            , last_name     : Set(req.last_name)
            , email         : Set(req.email)
            , phone         : Set(req.phone)
            , created_at    : Set(now)
            , updated_at    : Set(None)
        };

        let res = guest.insert(&self.db).await?;
        Ok(GuestSchemaOut {
            id              : res.id
            , first_name    : res.first_name
            , last_name     : res.last_name
            , email         : res.email
            , phone         : res.phone.unwrap_or_default()
            , created_at    : res.created_at
            , updated_at    : res.updated_at
        })
    }

    async fn get_guest(
        &self
        , id    : Uuid
    ) -> Result<Option<GuestSchemaOut>, DbErr> {
        let res = guests::Entity::find_by_id(id)
            .one(&self.db)
            .await?;

        Ok(res.map(|g| GuestSchemaOut {
            id              : g.id
            , first_name    : g.first_name
            , last_name     : g.last_name
            , email         : g.email
            , phone         : g.phone.unwrap_or_default()
            , created_at    : g.created_at
            , updated_at    : g.updated_at
        }))
    }

    async fn list_guests(&self) -> Result<Vec<GuestSchemaOut>, DbErr> {
        let res = guests::Entity::find()
            .all(&self.db)
            .await?;

        Ok(res.into_iter().map(|g| GuestSchemaOut {
            id              : g.id
            , first_name    : g.first_name
            , last_name     : g.last_name
            , email         : g.email
            , phone         : g.phone.unwrap_or_default()
            , created_at    : g.created_at
            , updated_at    : g.updated_at
        }).collect())
    }

    async fn update_guest(
        &self
        , id    : Uuid
        , req   : GuestSchemaIn
    ) -> Result<Option<GuestSchemaOut>, DbErr> {
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
        
        let guest = match guests::Entity::find_by_id(id).one(&self.db).await? {
            Some(g) => g,
            None    => return Ok(None),
        };

        let mut guest: guests::ActiveModel = guest.into();

        guest.first_name    = Set(req.first_name);
        guest.last_name     = Set(req.last_name);
        guest.email         = Set(req.email);
        guest.phone         = Set(req.phone);
        guest.updated_at    = Set(Some(now));

        let updated: guests::Model = guest.update(&self.db).await?;

        Ok(Some(GuestSchemaOut {
            id              : updated.id
            , first_name    : updated.first_name
            , last_name     : updated.last_name
            , email         : updated.email
            , phone         : updated.phone.unwrap_or_default()
            , created_at    : updated.created_at
            , updated_at    : updated.updated_at
        }))
    }

    async fn delete_guest(
        &self
        , id    : Uuid
    ) -> Result<bool, DbErr> {
        let res = guests::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        Ok(res.rows_affected > 0)
    }
}