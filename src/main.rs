use std::sync::Arc;
use axum::{Router, Json, middleware};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use log::{info, warn};
use sqlx::pool;
use crate::conf::database::get_pool;
use crate::conf::redis_db::redis_client;
use crate::controllers::auth_controller::AuthController;
use crate::controllers::auth_controller_impl::AuthControllerImpl;
use crate::dto::login_request::LoginRequest;
use crate::dto::register_request::RegisterRequest;
use crate::repositories::auth_repository_impl::AuthRepositoryImpl;
use crate::services::auth_service::AuthService;
use crate::services::auth_service_impl::AuthServiceImpl;
use chrono::{Utc, Duration};
use http::StatusCode;
use crate::conf::middleware::middleware_jwt;

mod conf {
    pub mod database;
    pub mod jwt;
    pub mod redis_db;
    pub mod middleware;
}
mod models { pub mod user; }
mod dto {
    pub mod login_request;
    pub mod register_request;
    pub mod web_response;
    pub mod login_response;
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
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let pool = get_pool().await.expect("failed to make connection database");

    let auth_repository = Arc::new(AuthRepositoryImpl::new(pool.clone()));
    let auth_service = Arc::new(AuthServiceImpl::new(auth_repository.clone()));
    let auth_controller = Arc::new(AuthControllerImpl::new(auth_service.clone()));

    let app = Router::new()
        .route("/hello", get(say_hello).layer(middleware::from_fn(middleware_jwt)))
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
    info!("This Server Ready");
    axum::serve(listener, app).await.unwrap();
}

async fn say_hello() -> impl IntoResponse {
    (StatusCode::OK, "Hello, world!")
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;
    use tokio::time::sleep;
    use crate::conf::jwt::{gen_jwt, validate_jwt};
    use crate::conf::redis_db::redis_client;

    #[test]
    fn jwt_testing() {
        let user_name = "korium";
        let ini_jwt = gen_jwt(user_name, chrono::Duration::seconds(10)).unwrap();
        println!("TOKEN JWT {:}", &ini_jwt);

        //EXTRACT JWT IF NOT Expired
        match validate_jwt(&ini_jwt) {
            Ok(claims) => println!("JWT BERHASIL DIEKSTRAK: {:?}", claims),
            Err(e) => println!("OH shit got err"),
        }

        //TEST HARUSNYA ERROR
        thread::sleep(Duration::from_secs(15));
        match validate_jwt(&ini_jwt) {
            Err(e) => println!("Sesuai harapan, token kadaluarsa: {}", e),
            Ok(_) => println!("ERROR: Seharusnya token sudah kadaluarsa!"),
        }
    }

    /*
     NEXT WE DO
     TEST MAKE 2 JWT TOKEN ALIVE IN 15 ADN LIVE IN 1 DAY
     JIKA LOGOUT SET BLACKLIST
     JIKA LOUUT ATAU SUDAH 15M / ATAU TOKEN DI TOKEN ADA DI REDIS KEMABLIKAN TOKEN

    */

    #[tokio::test]
    async fn test_redis_cache() {
        use redis::AsyncCommands;

        let mut conn = redis_client().await.unwrap();

        let _: () = conn.set("name", "ekoo").await.unwrap();
        let value: String = conn.get("name").await.unwrap();

        sleep(Duration::from_secs(2)).await;
        assert_eq!("ekoo", value)
    }
}
