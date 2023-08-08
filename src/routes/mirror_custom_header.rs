use axum::http::HeaderMap;



// TODO: proper error handling was out of scope for this lesson - fix unwraps
// NOTE: the header map can return values for other header details such
    // as user agent - just replace key
pub(super) async fn mirror_custom_header(headers: HeaderMap) -> String {
    let message_value = headers.get("x-message").unwrap();
    let message = message_value.to_str().unwrap().to_string();
    return  message
}