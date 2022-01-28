#![warn(clippy::all)]

use warp::{http::Method, Filter};
use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;

mod routes;
mod types;
mod store;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "practical_rust_book=info,warp=error".to_owned());

    let store = store::Store::new("postgres://localhost:5432/rustwebdev").await;
    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "XX1",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )})
        );

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    // let add_answer = warp::post()
    //     .and(warp::path("comments"))
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and(warp::body::form())
    //     .and_then(routes::answer::add_answer);

    let routes = get_questions
        .or(update_question)
        .or(add_question)
        .or(delete_question)
        // .or(add_answer)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
