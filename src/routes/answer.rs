use std::collections::HashMap;
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::{
    answer::{Answer, AnswerId},
    pagination::extract_pagination,
    question::QuestionId,
};
use handle_errors::Error;

pub async fn get_answers(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Answer> = store.answers.read().await.values().cloned().collect();
        if pagination.start > pagination.end {
            return Err(warp::reject::custom(Error::StartGreaterThanEnd));
        }
        if res.len() < pagination.end {
            return Err(warp::reject::custom(Error::EndParamExceedsItemList));
        }
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Answer> = store.answers.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("question_id").unwrap().to_string()),
    };
    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
