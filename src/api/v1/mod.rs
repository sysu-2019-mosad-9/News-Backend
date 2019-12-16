mod base_controller;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    let routes_list = vec![base_controller::base_routes()];
    routes_list.into_iter().flatten().collect()
}
