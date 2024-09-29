use reqwest::{self, Error, Response};
use std::collections::HashMap;

pub async fn ping() {
    let url = "http://0.0.0.0:8080/api/v1/ping";

    match reqwest::get(url).await {
        Ok(res) => {
            println!("Status: {}", res.status());
            println!("Headers:\n{:#?}", res.headers());

            let body = res.text().await.unwrap();
            println!("Body:\n{}", body);
        },
        Err(err) => panic!("Error: {}", err)
    };
}

pub async fn auth(username: String, password: String) -> Result<Response, Error> {
    let url = "http://0.0.0.0:8080/api/v1/auth";
    let client = reqwest::Client::new();
    let mut map = HashMap::new();

    map.insert("username", username.trim());
    map.insert("password", password.trim());

    return client.post(url).json(&map).send().await;
}
