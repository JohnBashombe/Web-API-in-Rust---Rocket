#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
}

#[get("/<name>")]
fn get_user(name: String) -> Json<User> {
    return Json(User { name: name });
}

pub fn rocket() {
    rocket::ignite()
        .mount("/api/v1", routes![get_user])
        .launch();
}

fn main() {
    rocket();
}
