use axum::http::StatusCode;




pub(super) async fn always_errors() -> Result<(), StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}