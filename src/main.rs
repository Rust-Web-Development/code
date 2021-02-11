use warp::{Filter, filters::body::BodyDeserializeError, reject::Reject, Rejection, Reply, http::StatusCode};
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Question {
    id: String,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
    comments: Option<Vec<CommentId>>,
    upvotes: Option<Vec<String>>,
}

#[derive(Serialize)]
struct Answer {
    id: String,
    question_id: String,
    content: String,
    comments: Vec<CommentId>,
    upvotes: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CommentId(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Comment {
    id: String,
    content: String,
    related_to: String,
    relation_id: String,
}
#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<String, Question>>>,
    comments: Arc<RwLock<HashMap<String, Comment>>>,
}
#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}

impl Reject for Error {}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            comments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<String, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params.get("start").unwrap().parse::<usize>().map_err(Error::ParseError)?,
            end: params.get("end").unwrap().parse::<usize>().map_err(Error::ParseError)?,
        })
    }

    Err(Error::MissingParameters)
}

async fn get_questions(params: HashMap<String, String>, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    if params.len() > 0 {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

async fn update_question(id: String, store: Store, question: Question) -> Result<impl warp::Reply, warp::Rejection> { 
    match store.questions.write().get_mut(&id) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status(
        "Question updated",
        StatusCode::OK,
    ))
}


async fn delete_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> { 
    match store.questions.write().remove(&id) {
        Some(_) => (),
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status(
        "Question deleted",
        StatusCode::OK,
    ))
}

async fn add_question(store: Store, question: Question) -> Result<impl warp::Reply, warp::Rejection> { 
    store.questions.write().insert(question.clone().id, question);

    Ok(warp::reply::with_status(
        "Question added",
        StatusCode::OK,
    ))
}

async fn  add_comment(store: Store, params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> { 
    let comment = Comment {
        id: "CI001".to_string(),
        content: params.get("content").unwrap().to_string(),
        related_to: params.get("relatedTo").unwrap().to_string(),
        relation_id: params.get("relationId").unwrap().to_string(),
    };

    store.comments.write().insert(comment.clone().id, comment);
    
    Ok(warp::reply::with_status(
        "Comment added",
        StatusCode::OK,
    ))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {   
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);
    
    let add_comment = warp::post()
        .and(warp::path("comments"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(add_comment);

    let routes = get_questions.or(update_question).or(add_question).or(add_comment).or(delete_question).recover(return_error);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
