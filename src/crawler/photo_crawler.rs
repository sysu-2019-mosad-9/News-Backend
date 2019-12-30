use crate::db::model::photo_model::PhotoModel;

use dotenv::dotenv;

use std::env;
use std::error::Error;

pub fn craw_photo_splider(count: usize) -> Result<Vec<PhotoModel>, Box<dyn Error>> {
    dotenv().ok();
    let url = env::var("IMAGES_URL")?;
    println!("will request for {:?}", url.as_str());
    #[derive(Deserialize)]
    struct SpliderPhoto {
        imgurl: String,
    }
    let mut photo_list = vec![];
    for _ in 0..count {
        let mut res = reqwest::get(url.as_str())?;
        let raw = res.text()?;
        println!("raw: {:?}", raw);
        let photo: SpliderPhoto = serde_json::from_str(raw.as_str())?;
        photo_list.push(PhotoModel {
            photo_id: "?".to_owned(),
            photo_link: photo.imgurl,
        });
    }
    Ok(photo_list)
}

pub fn crawl_photo() -> Result<(), Box<dyn Error>> {
    Ok(())
}
