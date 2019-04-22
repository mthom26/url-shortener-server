#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;

#[get("/")]
fn index() -> JsonValue {
    json!({ "message": "Hello World!"})
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
