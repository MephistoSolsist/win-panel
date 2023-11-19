use axum::{routing::get, Router};
use reqwest::Client;

async fn ical() -> String {
    let ical_url = "https://p203-caldav.icloud.com.cn/published/2/MTE2MDg5MTA3NTIxMTYwOJ62pmQYH-orN1EZPCTNLzb42OJtGwf4PeI0ojg6fXjzh83-l1lgCdpnbRdCvNPzgjuJI3hmuh3AUfSvecozMv4";

    let client = Client::new();
    let response = client.get(ical_url).send().await.unwrap();

    let body = response.text().await.unwrap();
    body
}

pub fn web_router_setup() -> Router {
    Router::new().route("/ical", get(ical))
}
