// ╦  ┌─┐┬ ┬┌─┐┬─┐ Lzyor Studio
// ║  ┌─┘└┬┘│ │├┬┘ kosync-project
// ╩═╝└─┘ ┴ └─┘┴└─ https://lzyor.work/koreader/
// 2023 (c) Lzyor

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub const DEFAULT_ADDR: &str = "0.0.0.0:3000";
pub const DEFAULT_TREE_NAME: &str = "kosync";
pub const DEFAULT_DB_PATH: &str = "data/kosync";
pub const FIELD_LEN_LIMIT: usize = 4096;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProgressState {
    pub document: String,
    pub percentage: f32,
    pub progress: String,
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}

macro_rules! def_error {
    ($($name:ident = ($code:expr, $status:expr, $msg:expr)),+) => {
        pub enum Error {
            $($name = $code,)*
        }

        impl IntoResponse for Error {
            fn into_response(self) -> Response {
                match self {
                    $(Error::$name => ($status, Json(json!({"code": $code, "message": $msg}))).into_response(),)*
                }
            }
        }
    };
}

#[rustfmt::skip]
def_error!(
    Internal = (2000, StatusCode::INTERNAL_SERVER_ERROR, "Unknown server error."),
    Unauthorized = (2001, StatusCode::UNAUTHORIZED, "Unauthorized"),
    UserExists = (2002, StatusCode::PAYMENT_REQUIRED, "Username is already registered."),
    InvalidRequest = (2003, StatusCode::FORBIDDEN, "Invalid request"),
    DocumentFieldMissing = (2004, StatusCode::FORBIDDEN, "Field 'document' not provided.")
);
