use std::process::Command;
use std::io::{self, Write};

use futures_util::future::FutureExt;

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

    io::stdout().write_all(&s.stderr).unwrap();

    let s = Command::new("sqlx")
            .arg("database")
            .arg("create")
            .arg("--database-url")
            .arg(format!("postgres://{}:{}/{}", config.db_host, config.db_port, config.db_name))
            .output()
            .expect("sqlx command failed to start");

    // Exdcute DB commands to drop and create a new test database
    io::stdout().write_all(&s.stderr).unwrap();

    // set up a new store instance with a db connection pool
    let store = setup_store(&config).await?;

    // start the server and listen for a sender signal to shut it down
    let handler = oneshot(store).await;

    // create a test user to use throughout the tests
    let u = User {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
    };

    let token;

    print!("Running register_new_user...");
    let result = std::panic::AssertUnwindSafe(register_new_user(&u)).catch_unwind().await;
    match result {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running login...");
    match std::panic::AssertUnwindSafe(login(u)).catch_unwind().await {
        Ok(t) => {
            token = t;
            println!("✓");
        },
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }
    
    print!("Running post_question...");
    match std::panic::AssertUnwindSafe(post_question(token)).catch_unwind().await {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    let _ = handler.sender.send(1);

    Ok(())
}

async fn register_new_user(user: &User) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/registration")
        .json(&user)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await;

    assert_eq!(res.unwrap(), "Account added".to_string());

}

async fn login(user: User) -> Token {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/login")
        .json(&user)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);

    res
        .json::<Token>()
        .await
        .unwrap()
}

async fn post_question(token: Token) {
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
        .unwrap()
        .json::<QuestionAnswer>()
        .await
        .unwrap();

    assert_eq!(res.id, 1);
    assert_eq!(res.title, q.title);
}
