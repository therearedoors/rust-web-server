use std::collections::HashMap;
use tracing::{event, info, Level, instrument};
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::{
    answer::{Answer, NewAnswer},
    pagination::{Pagination, extract_pagination},
};
use handle_errors::Error;

#[instrument]
pub async fn get_answers(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, pagination = true);
    let pagination = Pagination::default();
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        let pagination = extract_pagination(params)?;
        //TODO! need default values for limit and offset
    }
    info!(pagination = false);
    let res: Vec<Answer> = match store.get_answers(pagination.limit, pagination.offset).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    };
    Ok(warp::reply::json(&res))
}

pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_answer(new_answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
