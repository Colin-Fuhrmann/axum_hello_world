use axum::Extension;

use super::SharedData;




pub(super) async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String {
    return shared_data.message;
}