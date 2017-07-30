use rocket_contrib::{Json,Value};

#[error(404)]
fn not_found() -> Json<Value> {
  Json(json!({
    "status": "error",
    "reason": "Resource was not found"
  }))
}