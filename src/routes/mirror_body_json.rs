use axum::Json;
use serde::{Serialize, Deserialize};



pub(super) async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    return Json(MirrorJsonResponse { message: body.message, message_from_server: "Hello from Axum Response".to_string() });
}


#[derive(Serialize, Deserialize)]
pub(super) struct MirrorJson {
    message: String
}


#[derive(Serialize)]
pub(super) struct MirrorJsonResponse {
    message: String,
    message_from_server: String
}