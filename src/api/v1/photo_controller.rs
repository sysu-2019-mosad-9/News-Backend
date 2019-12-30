use crate::crawler::photo_crawler::craw_photo_splider;
use rand::random;
use rocket::http::Status;
use rocket::Route;
use rocket_contrib::json::JsonValue;

#[get("/photo/entries?<count>", format = "json")]
fn get_photo_entries(count: usize) -> Result<JsonValue, Status> {
    if count > 15 {
        return Err(Status::BadRequest);
    }
    match craw_photo_splider(count) {
        Ok(photo_list) => {
            let data: Vec<_> = photo_list
                .iter()
                .map(|m| {
                    json!({
                    "image_link": m.photo_link.clone(),
                    })
                })
                .collect();
            Ok(json!({ "count": count, "data": data, }))
        }
        Err(e) => {
            eprintln!("craw photo error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/fake/photo/entries?<count>", format = "json")]
fn fake_get_photo_entries(count: usize) -> Result<JsonValue, &'static str> {
    let urls = vec![
        "https://img.moegirl.org/common/7/7e/215%E6%BB%A1%E7%A0%B4.png",
        "https://img.moegirl.org/common/thumb/2/2b/Liangyishi_houqi_A.jpg/300px-Liangyishi_houqi_A.jpg",
    ];
    let mut data = vec![];
    for i in 0..count {
        data.push(
            json!({ "title": format!("shiki{}", i), "image_link": urls[random::<usize>()%2], }),
        );
    }
    Ok(json!({ "count": count, "data": data, }))
}

pub fn photo_routes() -> Vec<Route> {
    routes![get_photo_entries]
}
