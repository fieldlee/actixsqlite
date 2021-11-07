#[macro_use]
extern crate diesel;

mod routes;
mod schema;
mod model;

use actix_web::{App,HttpResponse,HttpServer,Responder};
use diesel::{SqliteConnection, r2d2::{self,ConnectionManager}};
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("can not found database");
    let database_pool = Pool::builder().build(ConnectionManager::<SqliteConnection>::new(database_url)).expect("database pool error");
    HttpServer::new(move||{
        App::new().data(database_pool.clone())
        .service(routes::add_product)
        .service(routes::update_product)
        .service(routes::get_products)
        .service(routes::del_product)
    }).bind("0.0.0.0:8081")?.run().await
}


