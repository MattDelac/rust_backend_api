use axum::{
    routing::{get, post, delete},
    Router,
};
use std::net::SocketAddr;
use my_axum::config::database::init_pool;
use my_axum::components::index::routes as index;
// use my_axum::components::tasks::routes as tasks;


#[tokio::main]
async fn main() {
    let pool = init_pool().await;

    let app = Router::new()
        .route("/", get(index::hello_world))
        .route("/", delete(index::destroy))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}