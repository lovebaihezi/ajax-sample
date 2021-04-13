pub mod post {
    use actix_web::{post, HttpResponse};
    use serde_json::json;
    use std::collections::HashMap;
    use std::format;
    const CHINA: &'static [&'static str] = &["Beijing", "Shanghai", "HongKong"];
    const EARTH: &'static [&'static str] = &["china", "russia"];

    #[post("/start")]
    pub async fn start() -> HttpResponse {
        let mut m = HashMap::new();
        m.insert("china", CHINA);
        m.insert("earth", EARTH);
        let x = format!("{:?}", m.keys());
        HttpResponse::Ok().json(x)
    }

    #[post("/end")]
    pub async fn end(text: String) -> HttpResponse {
        let mut m = HashMap::new();
        m.insert("china", CHINA);
        m.insert("earth", EARTH);
        let res = m.get(&text.as_str()).unwrap();
        HttpResponse::Ok().json(res)
    }
    #[post("/username")]
    pub async fn username(username: String) -> HttpResponse {
        let mut m = HashMap::new();
        m.insert("abcd", "123456");
        m.insert("zscv", "123456");
        let user = m.contains_key(&username.as_str());
        if user {
            HttpResponse::Ok().body("user exists!")
        } else {
            HttpResponse::Ok().body("ok!")
        }
    }
    #[post("/query")]
    pub async fn query(name: String) -> HttpResponse {
        let mut m = HashMap::new();
        m.insert(
            "China",
            json!([
                ["xi'an", "China", "Shan xi", "Province", "city"],
                ["BeiJing", "China", "BeiJing", "City", "city"]
            ]),
        );
        m.insert(
            "Unit States",
            json!([["New York", "Unit States", "New York State", "State", "city"]]),
        );
        let res = m.get(&name.as_str()).unwrap();
        HttpResponse::Ok().json(res)
    }
}
