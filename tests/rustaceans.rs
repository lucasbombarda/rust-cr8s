use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use rocket::serde::json::serde_json::{json, Value};

const BASE_URL: &str = "http://127.0.0.1:8000";

fn create_test_rustacean(client: &Client) -> Response {
    let url = format!("{}/rustaceans", BASE_URL);
    let response = client
        .post(url)
        .json(&json!({
            "name": "Test",
            "email": "test@test.com"
        }))
        .send()
        .unwrap();
    response
}

fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let url = format!("{}/rustacean/{}", BASE_URL, rustacean["id"]);
    client.delete(url).send().unwrap();
}

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
    let response = create_test_rustacean(&client);
    assert_eq!(response.status(), StatusCode::CREATED);

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
fn test_view_rustacean() {
    let client = Client::new();
    let response = create_test_rustacean(&client);

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
    let response = create_test_rustacean(&client);
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
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
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    let response = client
        .delete(format!("{}/rustacean/{}", BASE_URL, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
