use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct PasswordOptions {
    pub length: u8,
    pub password_type: Option<Vec<PasswordTypes>>,
    pub no_ambiguous: Option<bool>,
    pub no_similar: Option<bool>,
    pub no_sequential: Option<bool>,
}

impl Default for PasswordOptions {
    fn default() -> Self {
        Self {
            length: 16,
            password_type: Some(default_password_type()),
            no_ambiguous: Some(false),
            no_similar: Some(false),
            no_sequential: Some(false),
        }
    }
}

pub(crate) fn default_password_type() -> Vec<PasswordTypes> {
    vec![
        PasswordTypes::Special {
            required: None,
        },
        PasswordTypes::Numbers {
            required: None,
        },
        PasswordTypes::Lowercase {
            required: None,
        },
        PasswordTypes::Uppercase {
            required: None,
        },
    ]
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) enum PasswordTypes {
    Special {
        required: Option<bool>,
    },
    Numbers {
        required: Option<bool>,
    },
    Lowercase {
        required: Option<bool>,
    },
    Uppercase {
        required: Option<bool>,
    },
}