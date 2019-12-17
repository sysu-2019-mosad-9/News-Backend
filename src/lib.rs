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
mod model;
mod schema;
mod service;

use rocket::Route;

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
