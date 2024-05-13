use reqwest::blocking::Client;

use rocket::serde::json::{serde_json::json, Value};

pub static BASE_URL: &'static str = "http://127.0.0.1:8000";

pub fn create_test_rustacean(client: &Client) -> Value {
    let url = format!("{}/rustaceans", BASE_URL);
    let response = client
        .post(url)
        .json(&json!({
            "name": "Test",
            "email": "test@test.com"
        }))
        .send()
        .unwrap();
    response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let url = format!("{}/rustacean/{}", BASE_URL, rustacean["id"]);
    client.delete(url).send().unwrap();
}
