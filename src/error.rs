use std::io;

use axum::{http::StatusCode, response::IntoResponse};
use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    IO(io::Error),

    #[from]
    Sql(sqlx::Error),

    #[from]
    Migrator(sqlx::migrate::MigrateError),

    #[from]
    Template(tera::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self)).into_response()
    }
}
