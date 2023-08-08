use axum::{Router, routing::{get, post}, body::Body, Json, extract::Path};
use serde::{Serialize, Deserialize};



pub fn create_routes() -> Router<() ,Body> {
    return Router::new()
        .route("/hello_world",get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))

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


pub async fn path_variables(Path(id): Path<i32>) -> String {
    return id.to_string()
}


pub async fn hard_coded_path() -> String {
    return "You got 15".to_string()
}