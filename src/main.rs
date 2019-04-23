#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::State;
use rocket_contrib::json::{JsonValue, Json};
use rocket_contrib::databases::redis;
use rocket_contrib::databases::redis::Commands;

mod hasher;
use hasher::Hasher;

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
    hasher: State<Hasher>,
    body: Json<RequestBody>
) -> JsonValue {
    let url = &body.url;
    let index = conn.get("index").unwrap();
    let hashed = hasher.hash(index);
    let _ : () = conn.set(&hashed, url).unwrap();
    let _ : () = conn.incr("index", 1).unwrap();
    json!({ "message": format!("Hashed: {}, Url: {}", &hashed, url) })
}

fn main() {
    rocket::ignite()
        .manage(Hasher::new())
        .attach(RedisDb::fairing())
        .mount("/", routes![index, post_convert])
        .launch();
}
