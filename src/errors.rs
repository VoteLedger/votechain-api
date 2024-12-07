use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ApiErrorResponse {
    #[display["missing token"]]
    NoToken,

    #[display["invalid token"]]
    InvalidToken,

    #[display["expired token"]]
    ExpiredToken,

    #[display["invalid signature"]]
    InvalidSignature,

    #[display["missing bearer token"]]
    MissingBearerToken,

    #[display["token mismatch"]]
    TokenMismatch,

    #[display["internal server error"]]
    InternalServerError,

    #[display["resource not found"]]
    NotFound,
}

impl error::ResponseError for ApiErrorResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiErrorResponse::NoToken => StatusCode::UNAUTHORIZED, // 401: Missing authentication token
            ApiErrorResponse::InvalidToken => StatusCode::FORBIDDEN, // 403: Invalid token provided
            ApiErrorResponse::ExpiredToken => StatusCode::from_u16(498).unwrap(), // 498: Token expired (special code)
            ApiErrorResponse::InvalidSignature => StatusCode::from_u16(495).unwrap(), // 495: Invalid signature (special code)
            ApiErrorResponse::MissingBearerToken => StatusCode::from_u16(499).unwrap(), // 499: Client closed request or missing bearer token (special code)
            ApiErrorResponse::TokenMismatch => StatusCode::from_u16(496).unwrap(), // 496: Token mismatch (special code)
            ApiErrorResponse::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR, // 500: Internal server error
            ApiErrorResponse::NotFound => StatusCode::NOT_FOUND, // 404: Resource
        }
    }
}
