use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{HeaderMap, Response, StatusCode},
    response::{AppendHeaders, Html, IntoResponse},
    Json,
};
use base64::prelude::*;
use sail_core::configuration::{Configuration, Settings};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html("<h1>Not Found</h1>"))
}

fn check_authorization(headers: HeaderMap, settings: Settings) -> Result<(), ()> {
    // TODO: check this

    let header = headers
        .get("Authorization")
        .ok_or(())?
        .to_str()
        .map_err(|_| ())?;

    if !header.starts_with("Basic ") {
        return Err(());
    }

    info!("got auth header {header}");

    let encoded = &header["Basic".len()..];
    info!("got basic header with token {encoded}");

    let (app, token) = decode_token(encoded);

    info!("app = {app}, token = {token}");

    if let Some(app) = settings.applications.iter().find(|a| a.name == app) {
        if token == app.token {
            return Ok(());
        }
    }

    Err(())
}

fn decode_token(encoded: &str) -> (String, String) {
    let decoded: String = String::from_utf8(BASE64_STANDARD.decode(encoded).unwrap()).unwrap();

    let (app, token) = decoded.split_once(":").unwrap();

    (app.to_owned(), token.to_owned())
}

pub async fn version_check(
    State(configuration): State<Arc<Configuration>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if check_authorization(headers, configuration.get()).is_err() {
        return unauthorized_response();
    }

    (
        StatusCode::OK,
        AppendHeaders([("Docker-Distribution-API-Version", "registry/2.0")]),
        Html("<h1>Registry API v2 is (partially) supported!</h1>"),
    )
        .into_response()
}

pub async fn initiate_upload(
    State(configuration): State<Arc<Configuration>>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> impl IntoResponse {
    if check_authorization(headers, configuration.get()).is_err() {
        return unauthorized_response();
    }

    let uuid = Uuid::new_v4();

    (
        StatusCode::ACCEPTED,
        AppendHeaders([
            ("Location", format!("/v2/{name}/blobs/uploads/{uuid}")),
            ("Docker-Upload-UUID", uuid.to_string()),
        ]),
    )
        .into_response()
}

pub async fn upload_blob(
    State(configuration): State<Arc<Configuration>>,
    headers: HeaderMap,
    Path((name, uuid)): Path<(String, Uuid)>,
) -> impl IntoResponse {
    if check_authorization(headers, configuration.get()).is_err() {
        return unauthorized_response();
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("TODO! <upload-blob> name = {name}, uuid = {uuid}",),
    )
        .into_response()
}

#[derive(Deserialize)]
pub struct CompleteUploadParams {
    digest: Option<String>,
}

pub async fn complete_upload(
    State(configuration): State<Arc<Configuration>>,
    headers: HeaderMap,
    Path((name, uuid)): Path<(String, Uuid)>,
    Query(params): Query<CompleteUploadParams>,
) -> impl IntoResponse {
    if check_authorization(headers, configuration.get()).is_err() {
        return unauthorized_response();
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!(
            "TODO! <complete-upload> name = {name}, uuid = {uuid}, digest = {:?}",
            params.digest
        ),
    )
        .into_response()
}

pub async fn upload_manifest(
    State(configuration): State<Arc<Configuration>>,
    headers: HeaderMap,
    Path((name, reference)): Path<(String, Uuid)>,
) -> impl IntoResponse {
    if check_authorization(headers, configuration.get()).is_err() {
        return unauthorized_response();
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("TODO! <upload-manifest> name = {name}, reference = {reference}",),
    )
        .into_response()
}

fn unauthorized_response() -> Response<Body> {
    (
        StatusCode::UNAUTHORIZED,
        AppendHeaders([(
            "WWW-Authenticate",
            "Basic realm=\"sail\", charset=\"UTF-8\"",
        )]),
        Json(json!({
            "errors": [
                {
                    "code": "UNAUTHORIZED",
                    "message": "authentication required"
                }
            ]
        })),
    )
        .into_response()
}
