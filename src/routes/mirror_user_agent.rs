use axum::{TypedHeader, headers::UserAgent};

pub(super) async fn mirror_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    return user_agent.to_string();
}