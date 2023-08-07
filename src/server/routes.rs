use axum::{Router, routing::{get, post}, body::Body, Json};
use serde::{Serialize, Deserialize};



pub fn create_routes() -> Router<() ,Body> {
    return Router::new()
        .route("/hello_world",get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
}


//TODO: Put this in its proper place
async fn hello_world() -> String {
    return "hello world".to_string();
}


async fn mirror_body_string(body: String) -> String {
    return  body;
}


async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    return Json(MirrorJsonResponse { message: body.message, message_from_server: "Hello from Axum Response".to_string() });
}


#[derive(Serialize, Deserialize)]
struct MirrorJson {
    message: String
}


#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String
}