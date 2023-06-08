use diesel::{sqlite::SqliteConnection, r2d2::{ConnectionManager, self}};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub async fn init_pool() -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    pool
}