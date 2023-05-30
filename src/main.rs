mod options;

use std::{env, net::Ipv4Addr};
use tracing::{info};
use warp::{http::Response, Filter};
use options::{PasswordOptions, PasswordTypes} ;
use pw_gen::password::{PasswordGenerator, generator::Generator};

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
            let password = crate::generate_password(options);
            Response::builder()
                .header("Content-Type", "text/plain")
                .body(password)
        })
        .with(warp::trace::request());

    warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, port)).await;
}



#[tracing::instrument]
fn generate_password (option: Option<PasswordOptions>) -> String {
    let options = option.unwrap_or(PasswordOptions::default());
    let length = options.length;
    info!("Generating password with length {} and options {:?}", length, options);
    let mut generator = &mut Generator::new(length as usize);

    for password_type in options.password_type.unwrap_or(options::default_password_type()) {
        match password_type {
            PasswordTypes::Special { required } => {
                generator = generator.with_special(required);
            }
            PasswordTypes::Numbers { required } => {
                generator = generator.with_numbers(required);
            }
            PasswordTypes::Lowercase { required } => {
                generator = generator.with_lowercase(required);
            }
            PasswordTypes::Uppercase { required } => {
                generator = generator.with_uppercase(required);
            }
        }
    }

    if options.no_ambiguous.unwrap_or(false) {
        generator = generator.without_ambiguous();
    }

    if options.no_similar.unwrap_or(false) {
        generator = generator.without_similar();
    }

    if options.no_sequential.unwrap_or(false) {
        generator = generator.without_sequential();
    }

    let password = generator.generate();


    info!("Generated password: {}", password);
    password
}
