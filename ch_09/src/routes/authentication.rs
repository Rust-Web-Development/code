use argon2::{self, Config};
use paseto::v2::local::{decrypt_paseto, local_paseto};
use rand::Rng;
use warp::{http::StatusCode, Filter};
use std::future;

use crate::store::Store;
use crate::types::account::{Session, Account, AccountId};

pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };

    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_user(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(
                handle_errors::Error::ArgonLibraryError(e),
            )),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub fn decrypt_token(token: String) -> Result<String, handle_errors::Error> {
    decrypt_paseto(&token, None, "RANDOM WORDS WINTER MACINTOSH PC".as_bytes())
        .map_err(|_| handle_errors::Error::CannotDecryptToken)
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: AccountId) -> String {
    let state = serde_json::to_string(&account_id).expect("Failed to serialize state");
    local_paseto(&state, None, "RANDOM WORDS WINTER MACINTOSH PC".as_bytes())
        .expect("Failed to create token")
}


pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization")
        .and_then(| token: String| {
            let token = match decrypt_token(token) {
                Ok(t) => t,
                Err(_) => return future::ready(Err(warp::reject::reject())),
            };

            future::ready(Ok(Session {
                account_id: AccountId(serde_json::from_str(&token).expect("Cannot"))
            }))
        })
}
