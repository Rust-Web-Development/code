use std::collections::HashMap;

use warp::http::StatusCode;
use tracing::{instrument, event, Level};

use handle_errors::Error;
use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, NewQuestion};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "practical_rust_book", Level::INFO, "querying questions");
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        event!(Level::INFO, pagination = true);
        let res: Vec<Question> = match store.get_questions().await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError)),
        };
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        event!(Level::INFO, pagination = false);
        let res: Vec<Question> = match store.get_questions().await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError)),
        };
        Ok(warp::reply::json(&res))
    }
}

// pub async fn update_question(
//     id: String,
//     store: Store,
//     question: Question,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     match store.questions.write().get_mut(&QuestionId(id)) {
//         Some(q) => *q = question,
//         None => return Err(warp::reject::custom(Error::QuestionNotFound)),
//     }

//     Ok(warp::reply::with_status("Question updated", StatusCode::OK))
// }

// pub async fn delete_question(
//     id: String,
//     store: Store,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     match store.questions.write().remove(&QuestionId(id)) {
//         Some(_) => (),
//         None => return Err(warp::reject::custom(Error::QuestionNotFound)),
//     }

//     Ok(warp::reply::with_status("Question deleted", StatusCode::OK))
// }

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res: Question = match store.add_question(new_question).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError)),
    };

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}
