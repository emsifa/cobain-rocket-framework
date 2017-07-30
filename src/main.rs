#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod repository;
mod entity;
mod route;
mod error_handler;

fn main() {
  rocket::ignite()
    .mount("/api", routes![
      route::index,
      route::user::lists,
      route::user::get,
      route::user::post,
      route::user::put,
      route::user::delete,
    ])
    .catch(errors![error_handler::not_found])
    .launch();
}
