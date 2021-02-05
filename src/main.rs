use warp::{Filter, reject::Reject, Rejection, Reply, http::StatusCode};
use serde::Serialize;
use std::{collections::HashMap};

#[derive(Serialize, Debug, Clone)]
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

#[derive(Serialize, Debug, Clone)]
struct CommentId(String);

#[derive(Serialize, Debug)]
struct Comment {
    id: CommentId,
    content: String,
    related_to: String,
    relation_id: String,
}
#[derive(Clone)]
struct Store {
    questions: HashMap<String, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }

    fn init(&mut self) -> Self {
        let question = Question::new("1", "How?", "Please help!", vec!["general"]);

        self.add_question(&question)
    }

    fn add_question(&mut self, question: &Question) -> Self {
        self.questions.insert(question.clone().id, question.clone());
        
        Self {
            questions: self.questions.clone(),
        }
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_questions(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let mut res = Vec::new();

    for (_, value) in store.questions {
        res.push(value);
    }

    Ok(warp::reply::json(&res))
}

impl Question {
    fn new(id: &str, title: &str, content: &str, tags: Vec<&str>) -> Self {
        Question {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            tags: Some(tags.into_iter().map(|x| x.to_string()).collect()),
            comments: None,
            upvotes: None,
        }
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND,
        ))
    }
}


#[tokio::main]
async fn main() {
    let mut store = Store::new();
    store.init();

    let store_filter = warp::any().map(move || store.clone());

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}