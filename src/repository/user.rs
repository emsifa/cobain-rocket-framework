use entity::user::{User};
use std::collections::HashMap;
use std::sync::Mutex;

// Data storage here
lazy_static! {
  static ref USERS: Mutex<HashMap<u32, User>> = Mutex::new(HashMap::new());
}

#[allow(dead_code)]
static mut LAST_ID: u32 = 0;

fn get_next_id() -> u32 {
  unsafe {
    LAST_ID + 1
  }
}

pub fn lists() -> Vec<User> {
  let mut users = Vec::<User>::new();
  #[allow(unused_variables)]
  for (id, user) in USERS.lock().unwrap().iter() {
    users.push(user.clone());
  }
  users
}

pub fn add(user: &mut User) {
  let next_id = get_next_id();
  user.id = next_id;
  USERS.lock().unwrap().insert(next_id, user.clone());
  unsafe {
    LAST_ID = next_id;
  }
}

pub fn find_by_id(id: u32) -> Option<User> {
  let users = USERS.lock().unwrap();
  let result = users.get(&id);
  match result {
      Some(user) => Some(user.clone()),
      None => None,
  }
}

pub fn update_by_id(id: u32, user: &mut User) {
  user.id = id;
  USERS.lock().unwrap().insert(id, user.clone());
}

pub fn delete_by_id(id: u32) {
  USERS.lock().unwrap().remove(&id);
}
