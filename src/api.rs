// ╦  ┌─┐┬ ┬┌─┐┬─┐ Lzyor Studio
// ║  ┌─┘└┬┘│ │├┬┘ kosync-project
// ╩═╝└─┘ ┴ └─┘┴└─ https://lzyor.work/koreader/
// 2023 (c) Lzyor

use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::json;
use tracing::{instrument, Level};

use crate::{
    db::DB,
    defs::{Error, ProgressState, FIELD_LEN_LIMIT},
    utils::{is_valid_field, is_valid_key_field, now_timestamp},
};

#[derive(Debug, Clone)]
pub struct Authed(pub String);

pub async fn auth<B>(
    State(db): State<DB>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, Error> {
    let headers = req.headers();
    let check = |name| {
        headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .filter(|v| v.len() <= FIELD_LEN_LIMIT && is_valid_field(v))
    };
    match (check("x-auth-user"), check("x-auth-key")) {
        (Some(user), Some(key)) => match db.get_user(user) {
            Ok(Some(k)) if k == key => {
                tracing::debug!("auth: {:?}", user);
                let user = user.to_owned();
                req.extensions_mut().insert(Authed(user));
                Ok(next.run(req).await)
            }
            Ok(_) => Err(Error::Unauthorized),
            Err(_) => Err(Error::Internal),
        },
        _ => Err(Error::Unauthorized),
    }
}

#[instrument(level = Level::DEBUG)]
pub async fn auth_user() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"authorized": "OK"})))
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    username: String,
    password: String,
}

#[instrument(skip(db), level = Level::DEBUG)]
pub async fn create_user(
    State(db): State<DB>,
    Json(data): Json<CreateUser>,
) -> Result<impl IntoResponse, Error> {
    if !is_valid_key_field(&data.username) || !is_valid_field(&data.password) {
        return Err(Error::InvalidRequest);
    }
    if let Ok(Some(_)) = db.get_user(&data.username) {
        return Err(Error::UserExists);
    }
    match db.put_user(&data.username, &data.password) {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(json!({"username": data.username})),
        )),
        Err(_) => Err(Error::Internal),
    }
}

// - // - // - // - // - // - //

#[instrument(skip(db), level = Level::DEBUG)]
pub async fn get_progress(
    State(db): State<DB>,
    Path(doc): Path<String>,
    Extension(Authed(user)): Extension<Authed>,
) -> Result<impl IntoResponse, Error> {
    if !is_valid_key_field(&doc) {
        return Err(Error::DocumentFieldMissing);
    }
    match db.get_doc(&user, &doc) {
        Ok(Some(value)) => Ok(Json(value).into_response()),
        Ok(None) => Ok(Json(json!({ "document": doc })).into_response()),
        Err(_) => Err(Error::Internal),
    }
}

#[instrument(skip(db), level = Level::DEBUG)]
pub async fn update_progress(
    State(db): State<DB>,
    Extension(Authed(user)): Extension<Authed>,
    Json(mut data): Json<ProgressState>,
) -> impl IntoResponse {
    data.timestamp = Some(now_timestamp());
    match db.put_doc(&user, &data.document, &data) {
        Ok(_) => Ok(Json(json!({
            "document": data.document,
            "timestamp": data.timestamp
        }))),
        Err(_) => Err(Error::Internal),
    }
}

#[instrument(level = Level::DEBUG)]
pub async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"state": "OK"})))
}
