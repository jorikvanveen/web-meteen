use axum::{response::Response, Json};
use meteen_model::Operation;

pub fn sync(Json(operations): Json<Vec<Operation>>) -> Response {
    todo!()
}
