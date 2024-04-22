use rocket_db_pools::Database;
use rocket_routes::rustaceans;

mod models;
mod repositories;
mod rocket_routes;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);

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
            ],
        )
        .attach(DbConn::init())
        .launch()
        .await
        .unwrap();
}
