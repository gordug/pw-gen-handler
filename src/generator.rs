use crate::options::{self, PasswordOptions, PasswordTypes};
use pw_gen::password::{PasswordGenerator, generator::Generator};
use tracing::{info};

#[tracing::instrument]
pub(crate) fn generate_password (option: Option<PasswordOptions>) -> String {
    let options = option.unwrap_or(PasswordOptions::default());
    let length = options.length;
    info!("Generating password with length {} and options {:#?}", length, options);
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

    password
}