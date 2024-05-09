use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::{json, Value};

const BASE_URL: &str = "http://127.0.0.1:8000";

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let url = format!("{}/rustaceans", BASE_URL);
    let response = client.get(url).send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let url = format!("{}/rustaceans", BASE_URL);
    let response = client
        .post(url)
        .json(&json!({
            "name": "Test create rustacean",
            "email": "test@test.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Test create rustacean",
            "email": "test@test.com",
            "created_at": rustacean["created_at"]
        })
    );
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let url = format!("{}/rustaceans", BASE_URL);
    let response = client
        .post(url)
        .json(&json!({
            "name": "Test view rustacean",
            "email": "test@test.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    let response = client
        .get(format!("{}/rustacean/{}", BASE_URL, rustacean["id"]))
        .send()
        .unwrap();
    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Test view rustacean",
            "email": "test@test.com",
            "created_at": rustacean["created_at"]
        })
    );
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let url = format!("{}/rustaceans", BASE_URL);
    let response = client
        .post(url)
        .json(&json!({
            "name": "Test update rustacean",
            "email": "test@test.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    let response = client
            .put(format!("{}/rustaceans/{}", BASE_URL, rustacean["id"]))
            .json(&json!({
            "name": "Test update rustacean 2",
            "email": "updated@test.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Test update rustacean 2",
            "email": "updated@test.com",
            "created_at": rustacean["created_at"]
        })
    );
}
