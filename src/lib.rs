#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod api;
mod crawler;
mod db;
mod error;
mod model;
mod schema;
mod service;

use rocket::Route;
use rocket_contrib::json::JsonValue;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "resource was not found"
    })
}

fn mounts() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/api/v1", api::v1::routes())]
}

pub fn rocket() -> rocket::Rocket {
    let mut instance = rocket::ignite();
    for (path, routes) in mounts() {
        instance = instance.mount(path, routes);
    }
    instance
}
