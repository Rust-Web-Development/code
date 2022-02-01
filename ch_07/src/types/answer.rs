use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub content: String,
    pub question_id: i32,
}
