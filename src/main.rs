use std::collections::HashMap;
use actix_web::{get, post, web, 
    App, Error, error,
    HttpResponse, 
    HttpServer, 
    Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

mod login_dto;
use login_dto::LoginRequest;

#[post("/login")]
async fn login(login_body: web::Json<LoginRequest>) -> HttpResponse {

    let login_request = [("grant_type", "password")
                , ("client_id", "backend")
                , ("client_secret", "v7Hjb3KeZYVLQUuRzhFxf0n1cRahjlbP")
                , ("username", &login_body.username)
                , ("password", &login_body.password)];

    let client = awc::Client::default();
    let mut response = client.post("https://idp.buydata.ru/realms/master/protocol/openid-connect/token")
    .send_form(&login_request)
    .await.unwrap();

    let payload = response.body().await.unwrap();
    
    HttpResponse::Ok().body(payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}