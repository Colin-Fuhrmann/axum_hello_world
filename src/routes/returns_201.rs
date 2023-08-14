use axum::{response::{Response, IntoResponse}, http::StatusCode};



pub(super) async fn returns_201() -> Response {

    (
        StatusCode::CREATED,
        "This is a 201".to_string()
    )
    .into_response()
}