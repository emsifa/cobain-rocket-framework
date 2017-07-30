
#[derive(Serialize, Clone)]
pub struct User {
  pub id: u32,
  pub email: String,
  pub name: String,
  pub active: bool
}

impl User {
  pub fn new(email: &str, name: &str, active: bool) -> User {
    User {
      id: 0,
      email: email.to_string(),
      name: name.to_string(),
      active: active
    }
  }
}