use std::fmt::Formatter;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone,Debug, Serialize,strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    // -- Auth errors.
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthCtxNotInRequestExt,
    // -- Model errors.
    TicketDeleteFailIdNotFound { id: u64},
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        // Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError){
        #[allow(unreachable_patterns)] // push to fallback statement below
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // -- Auth.
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthCtxNotInRequestExt => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }
            // -- Model.
            Self::TicketDeleteFailIdNotFound {..} => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }
            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR
                )
        }
    }
}
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR, // fallback error
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