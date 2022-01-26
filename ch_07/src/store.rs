// use parking_lot::RwLock;
// use std::collections::HashMap;
// use std::sync::Arc;
use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::Row;

use crate::types::{
    // answer::Answer,
    question::{Question, QuestionId, NewQuestion},
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection!"),
        };
    
        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(self) -> Result<Vec<Question>, sqlx::Error> {
        match sqlx::query("SELECT * from questions")
            .map(|row: PgRow| Question {
			    id: QuestionId(row.get("id")),
                title: row.get("title"),
			    content: row.get("content"),
                tags: row.get("tags"),
		    })
            .fetch_all(&self.connection)
            .await {
                Ok(questions) => Ok(questions),
                Err(e) => Err(e),
            }
    }

    pub async fn add_question(self, new_question: NewQuestion) -> Result<Question, sqlx::Error> {
        match sqlx::query("INSERT INTO questions (title, content) VALUES ($1, $2) RETURNING id, title, content, tags")
            .bind(new_question.title)
            .bind(new_question.content)
            .map(|row: PgRow| Question {
			    id: QuestionId(row.get("id")),
                title: row.get("title"),
			    content: row.get("content"),
                tags: row.get("tags"),
		    })
            .fetch_one(&self.connection)
            .await {
                Ok(question) => Ok(question),
                Err(e) => {
                    tracing::event!(tracing::Level::INFO, "{:?}", e);
                    Err(e)
                },
            }
    }

    // fn init() -> HashMap<QuestionId, Question> {
    //     let file = include_str!("../questions.json");
    //     serde_json::from_str(file).expect("can't read questions.json")
    // }
}
