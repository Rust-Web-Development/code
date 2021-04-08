use hyper::{Error, service::{make_service_fn, service_fn}};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
		let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let new_service = make_service_fn(move |_| {
        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                match_requests(req)
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn match_requests(
	req: Request<Body>,
) -> Result<Response<Body>, Error> {
	match (req.method(), req.uri().path()) {
			(&Method::GET, "/") => get_handler(&req).await,
			_ => {
			    Ok(Response::builder()
				.status(StatusCode::NOT_FOUND)
			        .body("Not Found".into())
				.unwrap()
                     )
			}
	}
}

async fn get_handler(_req: &Request<Body>) -> Result<Response<Body>, Error> {  
	//println!("{:?}", req.uri().query());  
	Ok(Response::builder()
		.header(header::CONTENT_TYPE, "application/json")
		.body(Body::from("Test"))
		.unwrap()
       )
}
