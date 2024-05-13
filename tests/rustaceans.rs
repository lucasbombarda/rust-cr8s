use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;
use common::{create_test_rustacean, delete_test_rustacean, BASE_URL};

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
    let rustacean = create_test_rustacean(&client);

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Test",
            "email": "test@test.com",
            "created_at": rustacean["created_at"]
        })
    );

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let response = client
        .get(format!("{}/rustacean/{}", BASE_URL, rustacean["id"]))
        .send()
        .unwrap();
    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Test",
            "email": "test@test.com",
            "created_at": rustacean["created_at"]
        })
    );

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let response = client
        .put(format!("{}/rustacean/{}", BASE_URL, rustacean["id"]))
        .json(&json!({
            "name": "Test update",
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
            "name": "Test update",
            "email": "updated@test.com",
            "created_at": rustacean["created_at"]
        })
    );

    delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let response = create_test_rustacean(&client);
    let rustacean = create_test_rustacean(&client);
    let response = client
        .delete(format!("{}/rustacean/{}", BASE_URL, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
