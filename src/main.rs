use repositories::RustaceanRepository;
use rocket_db_pools::{Connection, Database};

mod models;
mod repositories;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);

#[rocket::get("/rustaceans")]
async fn get_rustaceans(mut db: Connection<DbConn>) {
    RustaceanRepository::find_multiple(&mut db, 100)
        .await
        .unwrap();
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .attach(DbConn::init())
        .launch()
        .await
        .unwrap();
}
