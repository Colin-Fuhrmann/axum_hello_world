use axum::{Router, body::Body, routing::{get, post}, http::Method, Extension, middleware};
use tower_http::cors:: {Any, CorsLayer};

use hello_world::hello_world;

use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;

use path_variables::path_variables;
use hard_coded_path::hard_coded_path;
use query_params::query_params;

use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;

use middleware_message::middleware_message;
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;

use always_errors::always_errors;
use returns_201::returns_201;


mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod hard_coded_path;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;
mod returns_201;



#[derive(Clone)]
pub struct SharedData {
    message: String
}



pub fn create_routes() -> Router<() ,Body> {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {message: "Hello from shared data".to_string()};


    return Router::new()
        .route("/read_middleware_custom_header", get(read_middleware_custom_header))
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/hello_world",get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
}