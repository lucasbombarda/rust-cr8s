use reqwest::{blocking::Client, StatusCode};

use rocket::form::validate::Contains;
use serde_json::{json, Value};

pub mod common;

use common::{create_test_rustacean, delete_test_rustacean, BASE_URL};

fn create_test_crate(client: &Client) -> Value {
    let rustacean = create_test_rustacean(&client);

    let response = client
        .post(format!("{}/crates", BASE_URL))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "test_crate",
            "name": "Test Crate",
            "version": "0.1.0",
            "description": "A test crate"
        }))
        .send()
        .unwrap();

    delete_test_rustacean(&client, rustacean);
    response.json().unwrap()
}

fn delete_test_crate(client: &Client, crate_test: Value) {
    let url = format!("{}/crate/{}", BASE_URL, crate_test["id"]);
    client.delete(url).send().unwrap();
}

#[test]
fn test_create_crate() {
    let client = Client::new();

    let rustacean = create_test_rustacean(&client);

    let response = client
        .post(format!("{}/crates", BASE_URL))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Test",
            "version": "0.1.0",
            "description": "A test crate"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let crate_test: Value = response.json().unwrap();

    assert_eq!(
        crate_test,
        json!({
            "id": crate_test["id"],
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Test",
            "version": "0.1.0",
            "description": "A test crate",
            "created_at": crate_test["created_at"],
        })
    );

    delete_test_rustacean(&client, rustacean);
    delete_test_crate(&client, crate_test);
}

#[test]
fn test_list_crates() {
    let client = Client::new();
    let crate_1 = create_test_crate(&client);
    let crate_2 = create_test_crate(&client);

    let response = client.get(format!("{}/crates", BASE_URL)).send().unwrap();
    let crates = response.json::<Value>().unwrap();

    assert!(crates.as_array().unwrap().contains(&crate_1));
    assert!(crates.as_array().unwrap().contains(&crate_2));

    delete_test_crate(&client, crate_1);
    delete_test_crate(&client, crate_2);
}
