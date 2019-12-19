use crate::error::ServerError;
use rand::random;
use rocket::Route;
use rocket_contrib::json::JsonValue;
use std::error::Error;

#[get("/news/tabs", format = "json")]
fn get_base_tabs() -> JsonValue {
    let tabs = vec!["推荐", "时事", "科技", "体育"];
    json!({ "len": tabs.len(), "names": tabs, })
}

#[get("/news/<tab_index>/entries?<count>&<img_most>", format = "json")]
fn get_news_enrties(
    tab_index: usize,
    mut count: usize,
    img_most: usize,
) -> Result<JsonValue, &'static str> {
    let titles = vec![
        "“撕我？你不配！”博士妈妈在家长群发飙，骂战持续4小时……",
        "情商低？娃哈哈公主霸气换掉王力宏，背后算盘打得相当精",
        "如何看待江西女老师有纹身，被家长们联名要求辞退？",
        "90后CEO救人遇难，上百名群众自发送行，他的孩子才3岁......",
        "中国人烧给死人的纸扎，登上巴黎设计周，还被法国的博物馆收藏，老外：中国人太浪漫了",
        "游戏宵禁要实行？所有网游将23点到6点关闭服务器，你同意吗？",
    ];
    let urls = vec![
        "https://img.moegirl.org/common/7/7e/215%E6%BB%A1%E7%A0%B4.png",
        "https://img.moegirl.org/common/thumb/2/2b/Liangyishi_houqi_A.jpg/300px-Liangyishi_houqi_A.jpg",
    ];
    if tab_index > 4 {
        return Err("tab_index out of range");
    }
    if count > titles.len() {
        count = titles.len();
    }
    let mut data = vec![];
    for i in 0..count {
        let mut imgs = vec![];
        for j in 0..img_most {
            imgs.push(urls[random::<usize>() % 2]);
        }
        let detail_url = format!("/api/v1/news/detail/{}", i + 1);
        data.push(json!({ "id": i+1, "title": titles[i], "image_links": imgs, "detail_url": detail_url, }))
    }
    Ok(json!({ "count": count, "data": data}))
}

#[get("/news/details/<news_index>", format = "json")]
fn get_news_detail(news_index: usize) -> Result<JsonValue, &'static str> {
    let mut data = vec![];
    data.push(json!({ "type": "typography", "content": format!("测试文字铸排成功: index {}", news_index), }));
    data.push(json!({ "type": "image", "content": "https://zh.moegirl.org/File:215%E6%BB%A1%E7%A0%B4.png", }));
    Ok(json!({ "count": 2, "data": data}))
}

pub fn base_routes() -> Vec<Route> {
    routes![get_base_tabs, get_news_enrties, get_news_detail]
}
