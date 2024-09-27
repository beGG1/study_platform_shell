use reqwest;

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
