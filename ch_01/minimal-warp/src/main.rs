use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path("hello")
        .map(|| format!("Hello, World!"));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 1337))
        .await;
}
