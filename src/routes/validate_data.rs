use axum::Json;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}


pub(super) async fn validate_data(Json(user): Json<RequestUser>) {
    
}