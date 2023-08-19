use actix_web::{post, web, 
    App,
    HttpResponse, 
    HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

mod keycloak_service;
mod model;

use keycloak_service::{get_token, create_user};
use model::{UserLogin, UserRegister};

#[post("/login")]
async fn login(login_body: web::Json<UserLogin>) -> HttpResponse {
    return get_token(login_body).await;
}

#[post("/register")]
async fn register(register_body: web::Json<UserRegister>) -> HttpResponse {
    return create_user(register_body).await;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    env_logger::init_from_env(Env::default());
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(login)
            .service(register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}