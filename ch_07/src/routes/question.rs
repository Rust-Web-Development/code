use std::collections::HashMap;

use warp::http::StatusCode;
use tracing::{instrument, event, Level};

use handle_errors::Error;
use crate::store::Store;
use crate::types::pagination::{Pagination, extract_pagination};
use crate::types::question::{Question, NewQuestion};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "practical_rust_book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }

    match store.get_questions(pagination.limit, pagination.offset).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(_) => Err(warp::reject::custom(Error::DatabaseQueryError)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(_) => Err(warp::reject::custom(Error::DatabaseQueryError)),
    }
}


pub async fn delete_question(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(_) = store.delete_question(id).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError));
    }

    Ok(warp::reply::with_status(format!("Question {} deleted", id), StatusCode::OK))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(_) = store.add_question(new_question).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError));
    }

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}
