use std::fmt::Formatter;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    // -- Model errors.
    TicketDeleteFailIdNotFound { id: u64},
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        let response = (StatusCode::INTERNAL_SERVER_ERROR).into_response();

        response
    }
}

// region: --- Error boilerplate
// impl std::fmt::Display for Error {
//     fn fmt(&self, fmt: &mut Formatter) -> core::result::Result<(), std::> {
//         write!(fmt, "{self:?}")
//     }
// }
//
// impl std::error::Error for Error {}
// // endregion: --- Error boilerplate