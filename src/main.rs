use rocket_db_pools::Database;

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
                rocket_routes::rustaceans::get_rustaceans,
                rocket_routes::rustaceans::view_rustacean,
                rocket_routes::rustaceans::create_rustacean,
                rocket_routes::rustaceans::update_rustacean,
                rocket_routes::rustaceans::delete_rustacean,
            ],
        )
        .attach(DbConn::init())
        .launch()
        .await
        .unwrap();
}
