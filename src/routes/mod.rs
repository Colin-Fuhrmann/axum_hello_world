use axum::{Router, body::Body, routing::{get, post}};

use hello_world::hello_world;

use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;

use path_variables::path_variables;
use hard_coded_path::hard_coded_path;
use query_params::query_params;

use mirror_user_agent::mirror_user_agent;


mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod hard_coded_path;
mod query_params;
mod mirror_user_agent;




pub fn create_routes() -> Router<() ,Body> {
    return Router::new()
        .route("/hello_world",get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))

}