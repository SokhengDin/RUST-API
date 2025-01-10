use sea_orm::*;
use uuid::Uuid;
use chrono::{Utc, FixedOffset};
use crate::{
    models::bookings,
    schemas::booking::*,
    services::traits::BookingServiceTrait,
};

#[derive(Clone)]
pub struct BookingService {
    db  : DatabaseConnection
}

impl BookingService {
    pub fn new(db   : DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl BookingServiceTrait for BookingService {
    async fn create_booking(
        &self
        , req   : BookingSchemaIn
    ) -> Result<BookingSchemaOut, DbErr> {
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());

        let booking = bookings::ActiveModel {
            id              : Set(Uuid::new_v4())
            , room_id       : Set(req.room_id)
            , guest_id      : Set(req.guest_id)
            , check_in_date : Set(req.check_in_date)
            , check_out_date: Set(req.check_out_date)
            , total_price   : Set(req.total_price)
            , status        : Set(req.status)
            , created_at    : Set(now)
            , updated_at    : Set(None)
        };

        let res = booking.insert(&self.db).await?;
        
        Ok(BookingSchemaOut {
            id              : res.id
            , room_id       : res.room_id
            , guest_id      : res.guest_id
            , check_in_date : res.check_in_date
            , check_out_date: res.check_out_date
            , total_price   : res.total_price
            , status        : res.status
            , created_at    : res.created_at
            , updated_at    : res.updated_at
        })
    }

    async fn get_booking(
        &self
        , id    : Uuid
    ) -> Result<Option<BookingSchemaOut>, DbErr> {
        let res = bookings::Entity::find_by_id(id)
            .one(&self.db)
            .await?;

        Ok(res.map(|b| BookingSchemaOut {
            id              : b.id
            , room_id       : b.room_id
            , guest_id      : b.guest_id
            , check_in_date : b.check_in_date
            , check_out_date: b.check_out_date
            , total_price   : b.total_price
            , status        : b.status
            , created_at    : b.created_at
            , updated_at    : b.updated_at
        }))
    }

    async fn list_bookings(&self) -> Result<Vec<BookingSchemaOut>, DbErr> {
        let res = bookings::Entity::find()
            .all(&self.db)
            .await?;

        Ok(res.into_iter().map(|b| BookingSchemaOut {
            id              : b.id
            , room_id       : b.room_id
            , guest_id      : b.guest_id
            , check_in_date : b.check_in_date
            , check_out_date: b.check_out_date
            , total_price   : b.total_price
            , status        : b.status
            , created_at    : b.created_at
            , updated_at    : b.updated_at
        }).collect())
    }

    async fn update_booking(
        &self
        , id    : Uuid
        , req   : BookingSchemaIn
    ) -> Result<Option<BookingSchemaOut>, DbErr> {
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
        
        let booking = match bookings::Entity::find_by_id(id).one(&self.db).await? {
            Some(b) => b,
            None    => return Ok(None),
        };

        let mut booking: bookings::ActiveModel = booking.into();

        booking.room_id        = Set(req.room_id);
        booking.guest_id       = Set(req.guest_id);
        booking.check_in_date  = Set(req.check_in_date);
        booking.check_out_date = Set(req.check_out_date);
        booking.total_price    = Set(req.total_price);
        booking.status         = Set(req.status);
        booking.updated_at     = Set(Some(now));

        let updated = booking.update(&self.db).await?;

        Ok(Some(BookingSchemaOut {
            id              : updated.id
            , room_id       : updated.room_id
            , guest_id      : updated.guest_id
            , check_in_date : updated.check_in_date
            , check_out_date: updated.check_out_date
            , total_price   : updated.total_price
            , status        : updated.status
            , created_at    : updated.created_at
            , updated_at    : updated.updated_at
        }))
    }

    async fn delete_booking(
        &self
        , id    : Uuid
    ) -> Result<bool, DbErr> {
        let res = bookings::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;

        Ok(res.rows_affected > 0)
    }

    async fn get_guest_bookings(
        &self
        , guest_id: Uuid
    ) -> Result<Vec<BookingSchemaOut>, DbErr> {
        let res = bookings::Entity::find()
            .filter(bookings::Column::GuestId.eq(guest_id))
            .all(&self.db)
            .await?;

        Ok(res.into_iter().map(|b| BookingSchemaOut {
            id              : b.id
            , room_id       : b.room_id
            , guest_id      : b.guest_id
            , check_in_date : b.check_in_date
            , check_out_date: b.check_out_date
            , total_price   : b.total_price
            , status        : b.status
            , created_at    : b.created_at
            , updated_at    : b.updated_at
        }).collect())
    }

    async fn get_room_bookings(
        &self
        , room_id: Uuid
    ) -> Result<Vec<BookingSchemaOut>, DbErr> {
        let res = bookings::Entity::find()
            .filter(bookings::Column::RoomId.eq(room_id))
            .all(&self.db)
            .await?;

        Ok(res.into_iter().map(|b| BookingSchemaOut {
            id              : b.id
            , room_id       : b.room_id
            , guest_id      : b.guest_id
            , check_in_date : b.check_in_date
            , check_out_date: b.check_out_date
            , total_price   : b.total_price
            , status        : b.status
            , created_at    : b.created_at
            , updated_at    : b.updated_at
        }).collect())
    }
}