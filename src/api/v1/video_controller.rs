use rand::random;
use rocket::Route;
use rocket_contrib::json::JsonValue;

#[get("/video/entries?<count>", format = "json")]
fn get_video_entries(count: usize) -> Result<JsonValue, &'static str> {
    let urls = vec![
        "http://flv3.bn.netease.com/videolib3/1707/03/bGYNX4211/SD/bGYNX4211-mobile.mp4",
        "http://clips.vorwaerts-gmbh.de/big_buck_bunny.mp4",
        "http://vjs.zencdn.net/v/oceans.mp4",
    ];
    let mut data = vec![];
    for i in 0..count {
        data.push(json!({
        "id": format!("{}", i+1),
        "title": format!("videl{}", i+1),
        "author": format!("dasin{}", i+1),
        "video_link": urls[random::<usize>()%3],
        "n_good": random::<usize>()%10,
        "n_comment": random::<usize>()%100,
        }));
    }
    Ok(json!({ "count": count, "data": data, }))
}

pub fn video_routes() -> Vec<Route> {
    routes![get_video_entries]
}
