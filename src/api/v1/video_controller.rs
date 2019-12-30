use crate::crawler::video_crawler::craw_video_splider;
use rand::random;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::JsonValue;
#[get("/video/entries?<count>")]
fn get_video_entries(count: usize) -> Result<JsonValue, Status> {
    match craw_video_splider() {
        Ok(video_list) => {
            let mut data: Vec<_> = video_list
                .iter()
                .map(|m| {
                    let n_comment = random::<usize>() % 50;
                    let n_good = n_comment * 5 + random::<usize>() % 100;
                    json!({
                    "id": m.video_id.clone(),
                    "title": m.video_title.clone(),
                    "uploader": m.video_uploader.clone(),
                    "video_preview": m.video_cover_link.clone(),
                    "video_link": m.video_link.clone(),
                    "n_good": n_good,
                    "n_comment": n_comment,
                    })
                })
                .collect();
            loop {
                if data.len() <= count {
                    break;
                }
                data.pop();
            }
            Ok(json!({ "count": count, "data": data, }))
        }
        Err(_e) => Err(Status::InternalServerError),
    }
}

#[get("/fake/video/entries?<count>", format = "json")]
fn fake_get_video_entries(count: usize) -> Result<JsonValue, &'static str> {
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
        "uploader": format!("dasin{}", i+1),
        "video_preview": "https://img.moegirl.org/common/7/7e/215%E6%BB%A1%E7%A0%B4.png",
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
