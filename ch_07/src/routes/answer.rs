use std::collections::HashMap;
use warp::http::StatusCode;

use handle_errors::Error;

use crate::store::Store;
use crate::types::answer::Answer;

pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        content: params.get("content").unwrap().to_string(),
        question_id: params.get("questionId").unwrap().parse().unwrap(),
    };

    match store.add_answer(answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}
