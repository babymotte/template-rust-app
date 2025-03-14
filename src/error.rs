use axum::{http::StatusCode, response::IntoResponse};
use miette::Diagnostic;
use std::io;
use thiserror::Error;
use worterbuch_client::ConnectionError;

#[derive(Error, Debug, Diagnostic)]
pub enum {{crate_name | upper_camel_case}}Error {
    #[error("Worterbuch error: {0}")]
    WorterbuchError(#[from] ConnectionError),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("YAML parse error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

impl IntoResponse for {{crate_name | upper_camel_case}}Error {
    // TODO differentiate between error causes
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{self}")).into_response()
    }
}

pub type {{crate_name | upper_camel_case}}Result<T> = Result<T, {{crate_name | upper_camel_case}}Error>;
