#[macro_use] extern crate rocket;

use rocket::{Rocket, Build, routes};
use rocket::fairing::AdHoc;

use my_rocket::config::database::Db;
use my_rocket::components::index::routes as index;
use my_rocket::components::tasks::routes as tasks;

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    Db::get_one(&rocket).await
        .expect("database connection")
        .run(|conn| { conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations"); })
        .await;

    rocket
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::fairing())
    .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    .mount("/", routes![index::hello_world, index::destroy])
    .mount("/tasks", routes![tasks::create, tasks::list, tasks::get_single, tasks::delete])
}