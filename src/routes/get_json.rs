use axum::Json;
use serde::Serialize;


#[derive(Clone, Serialize)]
pub(super) struct Data {
    message: String,
    count: i32,
    username: String
}


pub(super) async fn get_json() -> Json<Data> {

    let data = Data{
        message: "I am in data".to_string(),
        count: 500,
        username: "hellooooo".to_string()
    };

    return Json(data);
}