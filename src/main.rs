use std::{env, net::Ipv4Addr};
use tracing::{info};
use warp::{http::Response, Filter};
use serde::Deserialize;
use pw_gen::password::{PasswordGenerator, generator::Generator};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

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

#[derive(Debug, Deserialize)]
struct PasswordOptions {
    length: u8,
    special: bool,
    numbers: bool,
    lowercase: bool,
    uppercase: bool,
    no_ambiguous: bool,
    no_similar: bool,
    no_sequential: bool,
}

#[tracing::instrument]
fn generate_password (option: Option<PasswordOptions>) -> String {
    let options = option.unwrap_or(PasswordOptions {
        length: 16,
        special: true,
        numbers: true,
        lowercase: true,
        uppercase: true,
        no_ambiguous: true,
        no_similar: true,
        no_sequential: true,
    });
    let length = options.length;
    info!("Generating password with length {} and options {:?}", length, options);
    let mut generator = &mut Generator::new(length as usize);

    if options.special {
        generator = generator.with_special(None);
    }

    if options.numbers {
        generator = generator.with_numbers(None);
    }

    if options.lowercase {
        generator = generator.with_lowercase(None);
    }

    if options.uppercase {
        generator = generator.with_uppercase(None);
    }

    if options.no_ambiguous {
        generator = generator.without_ambiguous();
    }

    if options.no_similar {
        generator = generator.without_similar();
    }

    if options.no_sequential {
        generator = generator.without_sequential();
    }

    let password = generator.generate();


    info!("Generated password: {}", password);
    password
}
