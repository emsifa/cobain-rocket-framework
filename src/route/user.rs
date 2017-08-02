use entity::user::{User};
use rocket_contrib::{Json,Value};
use repository;

#[derive(Deserialize)]
pub struct UserData {
  email: String,
  name: String,
  active: bool
}

#[get("/user")]
pub fn lists() -> Json<Value> {
  Json(json!({
    "status": "ok",
    "users": repository::user::lists()
  }))
}

#[get("/user/<id>")]
pub fn get(id: u32) -> Json<Value> {
  Json(json!({
    "status": "ok",
    "user": repository::user::find_by_id(id)
  }))
}

#[post("/user", data = "<user_data>")]
pub fn post(user_data: Json<UserData>) -> Json<Value> {
  let mut user = User::new(&user_data.email, &user_data.name, user_data.active);
  repository::user::add(&mut user);
  Json(json!({
    "status": "ok",
    "user": user
  }))
}

#[put("/user/<id>", data = "<user_data>")]
pub fn put(id: u32, user_data: Json<UserData>) -> Json<Value> {
  let user = repository::user::find_by_id(id);
  match user {
    None => {
      Json(json!({
        "status": "error",
        "message": "User not found."
      })) 
    },
    Some(_) => {
      let mut updated_user = User::new(&user_data.email, &user_data.name, user_data.active);
      repository::user::update_by_id(id, &mut updated_user);
      Json(json!({
        "status": "ok",
        "user": updated_user
      })) 
    }
  }
}

#[delete("/user/<id>")]
pub fn delete(id: u32) -> Json<Value> {
  let user = repository::user::find_by_id(id);
  match user {
    None => {
      Json(json!({
        "status": "error",
        "message": "User not found."
      })) 
    },
    Some(_) => {
      repository::user::delete_by_id(id);
      Json(json!({
        "status": "ok",
      }))
    }
  }
}