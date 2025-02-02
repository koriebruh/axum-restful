use axum::Router;
use axum::routing::get;

mod conf {pub mod database;}
mod models { pub mod user; }
mod dto {pub mod login_request; pub mod register_request;}
mod repositories { pub mod auth_repository; pub mod auth_repository_impl;}
mod services {pub mod auth_service; pub mod auth_service_impl;}
mod controllers {}

mod utils {pub mod errors; pub mod hash;}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
