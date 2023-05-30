mod generator;
mod options;


use std::{env, net::Ipv4Addr};
use warp::{http::Response, Filter};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        _ => 3000,
    };
    println!("Starting server on port {}", port);

    let routes = warp::get()
        .and(warp::path("api"))
        .and(warp::path("GeneratePassword"))
        .and(warp::body::json())
        .map(|options| {
            let password = generator::generate_password(options);
            Response::builder()
                .header("Content-Type", "text/plain")
                .body(password)
        })
        .with(warp::trace::request());

    warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, port)).await;
}
