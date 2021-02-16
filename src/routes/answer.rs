use std::collections::HashMap;
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::{
    answer::Answer,
};


pub async fn  add_answer(store: Store, params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> { 
    let answer = Answer {
        id: "CI001".to_string(),
        content: params.get("content").unwrap().to_string(),
        question_id: params.get("questionId").unwrap().to_string(),
    };

    store.answers.write().insert(answer.clone().id, answer);
    
    Ok(warp::reply::with_status(
        "Answer added",
        StatusCode::OK,
    ))
}