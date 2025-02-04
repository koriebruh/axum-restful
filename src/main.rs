use std::sync::Arc;
use axum::{Router, Json};
use axum::routing::{get, post};
use crate::conf::database::get_pool;
use crate::controllers::auth_controller::AuthController;
use crate::controllers::auth_controller_impl::AuthControllerImpl;
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;
use crate::repositories::auth_repository_impl::AuthRepositoryImpl;
use crate::services::auth_service::AuthService;
use crate::services::auth_service_impl::AuthServiceImpl;

mod conf { pub mod database; }
mod models { pub mod user; }
mod dto {
    pub mod login_request;
    pub mod register_request;
    pub mod web_response;
}
mod repositories {
    pub mod auth_repository;
    pub mod auth_repository_impl;
}
mod services {
    pub mod auth_service;
    pub mod auth_service_impl;
}
mod controllers {
    pub mod auth_controller;
    pub mod auth_controller_impl;
}

mod utils {
    pub mod errors;
    pub mod hash;
}


#[tokio::main]
async fn main() {
    env_logger::init();
    let pool = match get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to create database pool: {}", e);
            std::process::exit(1);
        }
    };

    let auth_repository = Arc::new(AuthRepositoryImpl::new(pool.clone()));
    let auth_service = Arc::new(AuthServiceImpl::new(auth_repository.clone()));
    let auth_controller = Arc::new(AuthControllerImpl::new(auth_service.clone()));

    let app = Router::new()
        .route("/login", post({
            let auth_controller = Arc::clone(&auth_controller);
            move |Json(req): Json<LoginRequest>| {
                let controller = Arc::clone(&auth_controller); // Cloning here
                async move { controller.login(Json(req)).await } // Ensuring async is awaited
            }
        }))
        .route("/register", post({
            let auth_controller = Arc::clone(&auth_controller);
            move |Json(req): Json<RegisterRequest>| {
                let controller = Arc::clone(&auth_controller); // Cloning here
                async move { controller.register(Json(req)).await } // Ensuring async is awaited
            }
        }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

