#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::{JsonValue, Json};
use rocket_contrib::databases::redis;

#[database("redis_logs")]
struct RedisDb(redis::Connection);

#[derive(Deserialize)]
struct RequestBody {
    url: String
}

#[get("/")]
fn index() -> JsonValue {
    json!({ "message": "Hello World!"})
}

#[post("/convert", data="<body>")]
fn post_convert(
    conn: RedisDb,
    body: Json<RequestBody>
) -> JsonValue {
    let url = &body.url;
    json!({ "message": format!("Received: {}", url) })
}

fn main() {
    rocket::ignite()
        .attach(RedisDb::fairing())
        .mount("/", routes![index, post_convert])
        .launch();
}
