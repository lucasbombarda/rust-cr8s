use rocket_db_pools::Database;
use rocket_routes::{crates, rustaceans};

mod models;
mod repositories;
mod rocket_routes;
mod schema;

#[rocket::main]
async fn main() {
    rocket::build()
        .mount(
            "/",
            rocket::routes![
                rustaceans::get_rustaceans,
                rustaceans::view_rustacean,
                rustaceans::create_rustacean,
                rustaceans::update_rustacean,
                rustaceans::delete_rustacean,
                crates::get_crates,
                crates::view_crate,
                crates::create_crate,
                crates::update_crate,
                crates::delete_crate,
            ],
        )
        .attach(rocket_routes::DbConn::init())
        .launch()
        .await
        .unwrap();
}
