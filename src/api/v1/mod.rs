mod base_controller;
mod photo_controller;
mod video_controller;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    let routes_list = vec![
        base_controller::base_routes(),
        photo_controller::photo_routes(),
        video_controller::video_routes(),
    ];
    routes_list.into_iter().flatten().collect()
}
