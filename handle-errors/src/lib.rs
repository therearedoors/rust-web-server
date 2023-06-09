use warp::{
    filters::{
        body::BodyDeserializeError,
        cors::CorsForbidden,
    },
    reject::Reject,
    Rejection,
    Reply,
    http::StatusCode
};

use sqlx::error::Error as SqlxError;

#[derive(Debug)]
pub enum Error {
ParseError(std::num::ParseIntError),
MissingParameters,
ItemNotFound,
StartGreaterThanEnd,
EndParamExceedsItemList,
DatabaseQueryError(SqlxError),
}

impl std::fmt::Display for Error {
fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
        Error::ParseError(ref err) => {
            write!(f, "Cannot parse parameter: {}", err)
        },
        Error::MissingParameters => write!(f, "Missing parameter"),
        Error::ItemNotFound => write!(f, "Item not found"),
        Error::EndParamExceedsItemList => write!(f, "End parameter exceeds item list length"),
        Error::StartGreaterThanEnd => write!(f, "Start cannot be greater than end parameter"),
        Error::DatabaseQueryError(ref e) => {
            write!(f, "Database query error {}", e)
        }
    }
}
}

impl Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
if let Some(error) = r.find::<Error>() {
  Ok(warp::reply::with_status(
    error.to_string(),
    StatusCode::RANGE_NOT_SATISFIABLE,
  ))
} else if let Some(error) = r.find::<CorsForbidden>() {
    Ok(warp::reply::with_status(
        error.to_string(),
        StatusCode::FORBIDDEN,
    ))
} else if let Some(error) = r.find::<BodyDeserializeError>() {
    Ok(warp::reply::with_status(
        error.to_string(),
        StatusCode::UNPROCESSABLE_ENTITY,
    ))
} else {
    Ok(warp::reply::with_status(
        "Route not found".to_string(),
        StatusCode::NOT_FOUND,
    ))
}
}