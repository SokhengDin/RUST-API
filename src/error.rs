use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::Serialize;
use sea_orm::DbErr;


#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code        : u16
    , pub message   : String
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] DbErr)
    , #[error("Hotel not found with ID: {0}")]
    HotelNotFound(String)
    , #[error("Room not found with ID: {0}")]
    RoomNotFound(String),
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = match &self {
            ApiError::Database(db_err) => match db_err {
                DbErr::RecordNotFound(_) => Status::NotFound,
                DbErr::Query(err) if err.to_string().contains("violates foreign key constraint") => Status::NotFound,
                _ => Status::InternalServerError,
            },
            ApiError::HotelNotFound(_) | ApiError::RoomNotFound(_) => Status::NotFound,
        };

        let error = ErrorResponse {
            code: status.code,
            message: self.to_string(),
        };

        Json(error).respond_to(req)
    }
}