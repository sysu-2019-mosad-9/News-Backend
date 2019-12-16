use rocket::Route;
use rocket_contrib::json::JsonValue;

#[get("/base/tabs", format = "json")]
fn get_base_tabs() -> JsonValue {
    let tabs = vec!["推荐", "时事", "科技"];
    json!({ "len": tabs.len(), "names": tabs, })
}

#[get("/base/news/titles?<count>", format = "json")]
fn get_base_titles(mut count: usize) -> JsonValue {
    let titles = vec![
        "“撕我？你不配！”博士妈妈在家长群发飙，骂战持续4小时……",
        "情商低？娃哈哈公主霸气换掉王力宏，背后算盘打得相当精",
        "如何看待江西女老师有纹身，被家长们联名要求辞退？",
        "90后CEO救人遇难，上百名群众自发送行，他的孩子才3岁......",
        "中国人烧给死人的纸扎，登上巴黎设计周，还被法国的博物馆收藏，老外：中国人太浪漫了",
        "游戏宵禁要实行？所有网游将23点到6点关闭服务器，你同意吗？"
    ];
    if count > titles.len() {
        count = titles.len();
    }
    json!({ "count": count, "titles": &titles[0..count], })
}

pub fn base_routes() -> Vec<Route> {
    routes![get_base_tabs, get_base_titles,]
}
