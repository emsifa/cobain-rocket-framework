use rocket_contrib::{Json,Value};

pub mod user;

#[get("/")]
pub fn index() -> Json<Value> {
  Json(json!({
    "status": "ok",
    "message": "Hello World"
  }))
}