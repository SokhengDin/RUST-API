use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::EnumIter;
use sea_orm_migration::prelude::extension::postgres::Type;

#[derive(Iden)]
pub enum BookingStatus {
    #[iden = "booking_status"]
    Enum,
    #[iden = "pending"]
    Pending,
    #[iden = "confirmed"]
    Confirmed,
    #[iden = "cancelled"]
    Cancelled,
    #[iden = "completed"]
    Completed,
}


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        // Create the enum type
        manager
            .create_type(
                Type::create()
                    .as_enum(BookingStatus::Enum)
                    .values([
                        BookingStatus::Pending,
                        BookingStatus::Confirmed,
                        BookingStatus::Cancelled,
                        BookingStatus::Completed,
                    ])
                    .to_owned(),
            )
            .await?;

        // Create hotels table
        manager
            .create_table(
                Table::create()
                    .table(Hotels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hotels::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Hotels::Name).string().not_null())
                    .col(ColumnDef::new(Hotels::Address).text().not_null())
                    .col(ColumnDef::new(Hotels::Rating).double().not_null())
                    .col(ColumnDef::new(Hotels::Description).text().null())
                    .col(ColumnDef::new(Hotels::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Hotels::UpdatedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;

        // Create rooms table
        manager
            .create_table(
                Table::create()
                    .table(Rooms::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rooms::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Rooms::HotelId).uuid().not_null())
                    .col(ColumnDef::new(Rooms::RoomNumber).string().not_null())
                    .col(ColumnDef::new(Rooms::RoomType).string().not_null())
                    .col(ColumnDef::new(Rooms::PricePerNight)
                        .decimal_len(10, 2)
                        .not_null())
                    .col(ColumnDef::new(Rooms::IsAvailable).boolean().not_null())
                    .col(ColumnDef::new(Rooms::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Rooms::UpdatedAt).timestamp_with_time_zone().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rooms_hotel")
                            .from(Rooms::Table, Rooms::HotelId)
                            .to(Hotels::Table, Hotels::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Create guests table
        manager
            .create_table(
                Table::create()
                    .table(Guests::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Guests::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Guests::FirstName).string().not_null())
                    .col(ColumnDef::new(Guests::LastName).string().not_null())
                    .col(ColumnDef::new(Guests::Email).string().not_null())
                    .col(ColumnDef::new(Guests::Phone).string().null())
                    .col(ColumnDef::new(Guests::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Guests::UpdatedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;

        // Create bookings table
        manager
            .create_table(
                Table::create()
                    .table(Bookings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bookings::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bookings::RoomId).uuid().not_null())
                    .col(ColumnDef::new(Bookings::GuestId).uuid().not_null())
                    .col(ColumnDef::new(Bookings::CheckInDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Bookings::CheckOutDate).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Bookings::TotalPrice)
                        .decimal_len(10, 2)
                        .not_null())
                    .col(ColumnDef::new(Bookings::Status)
                        .custom(Alias::new("booking_status"))
                        .not_null())
                    .col(ColumnDef::new(Bookings::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Bookings::UpdatedAt).timestamp_with_time_zone().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bookings_room")
                            .from(Bookings::Table, Bookings::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bookings_guest")
                            .from(Bookings::Table, Bookings::GuestId)
                            .to(Guests::Table, Guests::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(Bookings::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Guests::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Rooms::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Hotels::Table).to_owned())
            .await?;
        
        // Drop the enum type last
        manager
            .drop_type(
                Type::drop()
                    .name(Alias::new("booking_status"))
                    .to_owned()
            )
            .await?;

        Ok(())

    }
}

#[derive(Iden)]
enum Hotels {
    Table,
    Id,
    Name,
    Address,
    Rating,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Rooms {
    Table,
    Id,
    HotelId,
    RoomNumber,
    RoomType,
    PricePerNight,
    IsAvailable,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Guests {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Phone,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Bookings {
    Table,
    Id,
    RoomId,
    GuestId,
    CheckInDate,
    CheckOutDate,
    TotalPrice,
    Status,
    CreatedAt,
    UpdatedAt,
}