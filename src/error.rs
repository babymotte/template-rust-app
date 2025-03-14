/*
 *  Copyright (C) 2025 Michael Bachmann
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
