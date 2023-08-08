use axum::{
    routing::get,
    Router
};

use axum_hello_world::run_server;



#[tokio::main]
async fn main() {
    
    run_server().await
}