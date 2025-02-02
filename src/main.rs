mod conf {}
mod models { pub mod user; }
mod dto {pub mod login_request; pub mod register_request;}
mod repositories { pub mod auth_repository; pub mod auth_repository_impl;}
mod services {}
mod controllers {}

mod utils {pub mod errors; pub mod hash;}

fn main() {
    println!("Hello, world!");
}
