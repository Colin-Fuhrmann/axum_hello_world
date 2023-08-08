use axum::extract::Path;



pub(super) async fn path_variables(Path(id): Path<i32>) -> String {
    return id.to_string()
}