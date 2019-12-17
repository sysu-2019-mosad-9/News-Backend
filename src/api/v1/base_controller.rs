use rand::random;
use rocket::Route;
use rocket_contrib::json::JsonValue;

#[get("/news/tabs", format = "json")]
fn get_base_tabs() -> JsonValue {
    let tabs = vec!["推荐", "时事", "科技"];
    json!({ "len": tabs.len(), "names": tabs, })
}

#[get("/news/<tab_index>/titles?<count>", format = "json")]
fn get_base_titles(tab_index: usize, mut count: usize) -> JsonValue {
    let titles = vec![
        "“撕我？你不配！”博士妈妈在家长群发飙，骂战持续4小时……",
        "情商低？娃哈哈公主霸气换掉王力宏，背后算盘打得相当精",
        "如何看待江西女老师有纹身，被家长们联名要求辞退？",
        "90后CEO救人遇难，上百名群众自发送行，他的孩子才3岁......",
        "中国人烧给死人的纸扎，登上巴黎设计周，还被法国的博物馆收藏，老外：中国人太浪漫了",
        "游戏宵禁要实行？所有网游将23点到6点关闭服务器，你同意吗？",
    ];
    if count > titles.len() {
        count = titles.len();
    }
    json!({ "count": count, "titles": &titles[0..count], })
}

#[get("/news/<tab_index>/images?<count>&<most>")]
fn get_base_images(tab_index: usize, count: usize, most: usize) -> Result<JsonValue, &'static str> {
    let url = vec![
        "https://zh.moegirl.org/File:215%E6%BB%A1%E7%A0%B4.png",
        "https://img.moegirl.org/common/thumb/2/2b/Liangyishi_houqi_A.jpg/300px-Liangyishi_houqi_A.jpg",
    ];
    if count > 10 || most > 5 {
        return Err("you are requesting too many images !");
    }
    let mut imgs_list = vec![];
    for _i in 0..count {
        let mut imgs = vec![];
        for _j in 0..most {
            let w = random::<usize>() % 2;
            imgs.push(url[w]);
        }
        imgs_list.push(json!({ "each": most, "urls": imgs, }));
    }
    Ok(json!({ "count": count, "payload": imgs_list, }))
}

pub fn base_routes() -> Vec<Route> {
    routes![get_base_tabs, get_base_titles, get_base_images,]
}
