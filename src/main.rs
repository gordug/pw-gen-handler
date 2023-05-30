mod generator;
mod options;


use std::{env, net::Ipv4Addr};
use tracing::{log::info};
use tracing_subscriber::{EnvFilter};
use warp::{http::Response, Filter};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    let format =
        tracing_subscriber::fmt::format()
            .with_level(true)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_ansi(true)
            .compact();

    tracing_subscriber::fmt()
        .event_format(format)
        .init();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_filter_reloading()
        .finish();

    // Get port from environment variable
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";

    // Default to port 3000
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        _ => 3000,
    };

    // Create routes for warp
    let routes = warp::get()
        // /api/GeneratePassword
        .and(warp::path("api"))
        .and(warp::path("GeneratePassword"))

        // Deserialize JSON body into PasswordOptions
        .and(warp::body::json())

        // Generate password
        .map(|options| {
            let password = generator::generate_password(options);
            Response::builder()
                .header("Content-Type", "text/plain")
                .body(password)
        })

        // Add tracing
        .with(warp::trace::request());

    // Start server
    warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, port)).await;
}
