use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: String,
    pub content: String,
    pub question_id: String,
}