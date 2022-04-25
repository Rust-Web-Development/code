use std::process::Command;
use std::io::{self, Write};

use rust_web_dev::{config, handle_errors, oneshot, setup_store};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Question {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct QuestionAnswer {
    id: i32,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Token(String);

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenv::dotenv().ok();
    let config = config::Config::new().expect("Config can't be set");

    let s = Command::new("sqlx")
        .arg("database")
        .arg("drop")
        .arg("--database-url")
        .arg(format!("postgres://{}:{}/{}", config.db_host, config.db_port, config.db_name))
        .arg("-y")
        .output()
        .expect("sqlx command failed to start");

    io::stdout().write_all(&s.stdout).unwrap();
    io::stdout().write_all(&s.stderr).unwrap();

    let s = Command::new("sqlx")
            .arg("database")
            .arg("create")
            .arg("--database-url")
            .arg(format!("postgres://{}:{}/{}", config.db_host, config.db_port, config.db_name))
            .output()
            .expect("sqlx command failed to start");

    io::stdout().write_all(&s.stdout).unwrap();
    io::stdout().write_all(&s.stderr).unwrap();

    let store = setup_store(&config).await?;

    let handler = oneshot(store).await;

    let u = User {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
    };

    register_new_user(&u).await?;
    let token = login(u).await?;
    post_question(token).await?;

    let _ = handler.sender.send(1);

    Ok(())
}

async fn register_new_user(user: &User) -> Result<(), handle_errors::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/registration")
        .json(&user)
        .send()
        .await
        .map_err(handle_errors::Error::ReqwestAPIError)?
        .json::<Value>()
        .await
        .map_err(handle_errors::Error::ReqwestAPIError)?;

    assert_eq!(res, "Account added".to_string());

    Ok(())
}

async fn login(user: User) -> Result<Token, handle_errors::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/login")
        .json(&user)
        .send()
        .await
        .map_err(handle_errors::Error::ReqwestAPIError)?;

    assert_eq!(res.status(), 200);

    let token = res
        .json::<Token>()
        .await
        .map_err(handle_errors::Error::ReqwestAPIError)?;

    Ok(token)
}

async fn post_question(token: Token) -> Result<(), handle_errors::Error> {
    let q = Question {
        title: "First Question".to_string(),
        content: "How can I test?".to_string(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/questions")
        .header("Authorization", token.0)
        .json(&q)
        .send()
        .await
        .map_err(handle_errors::Error::ReqwestAPIError)?
        .json::<QuestionAnswer>()
        .await
        .map_err(handle_errors::Error::ReqwestAPIError)?;

    assert_eq!(res.id, 1);
    assert_eq!(res.title, q.title);

    Ok(())
}