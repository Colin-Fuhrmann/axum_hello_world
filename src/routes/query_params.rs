use axum::{extract::Query, Json};
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    message: String,
    id: i32
}




pub(super) async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    return Json(query)
}


