use diesel::{prelude::*, r2d2};
use actix_web::{get, post, web, middleware, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;

use my_actix::components::index::routes as index;
use my_actix::components::tasks::routes as tasks;

/// Short-hand for the database pool type to use throughout the app.
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    let pool = initialize_db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/")
                    .service(index::hello_world)
                    .service(index::destroy)
            )
            .service(
                web::scope("/tasks")
                    .service(tasks::create)
                    .service(tasks::list)
                    .service(tasks::get_single)
                    .service(tasks::delete)
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
