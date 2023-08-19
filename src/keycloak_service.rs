use actix_web::{HttpResponse, web};
use serde_json::json;

use crate::model::{UserLogin, UserRegister};


async fn get_admin_token() -> String {
    let login_request = 
        [("grant_type", "client_credentials")
        , ("client_id", "backend")
        , ("client_secret", "v7Hjb3KeZYVLQUuRzhFxf0n1cRahjlbP")];

    let client = awc::Client::default();
    let mut response = client.post("https://idp.buydata.ru/realms/master/protocol/openid-connect/token")
        .send_form(&login_request)
        .await.unwrap();

    let body = response.json::<serde_json::Value>().await.unwrap();

    let token = &body["access_token"].as_str().unwrap().to_string();

    let bearer: String = token.to_owned();
        
    return bearer;
}

pub async fn get_token(login: web::Json<UserLogin>) -> HttpResponse {
    let login_request = [("grant_type", "password")
    , ("client_id", "backend")
    , ("client_secret", "v7Hjb3KeZYVLQUuRzhFxf0n1cRahjlbP")
    , ("username", &login.username)
    , ("password", &login.password)];

    let client = awc::Client::default();
    let mut response = client.post("https://idp.buydata.ru/realms/master/protocol/openid-connect/token")
        .send_form(&login_request)
        .await.unwrap();

    let body = response.json::<serde_json::Value>().await.unwrap();
        
    HttpResponse::Ok().json(body)
}

pub async fn create_user(user_body: web::Json<UserRegister>) -> HttpResponse {
    
    let data = json!({
        "firstName": user_body.first_name,
        "lastName": user_body.last_name,
        "username": user_body.username,
        "email": user_body.email,
        "enabled": true,
        "credentials": [
            {
                "type": "password",
                "value": user_body.password,
                "temporary": false
            }
        ],
        "realmRoles": ["USER"]
    });

    let bearer_token: String = get_admin_token().await;

    let client = awc::Client::default();
    let mut response = client.post("https://idp.buydata.ru/admin/realms/master/users")
                            .bearer_auth(&bearer_token)
                            .send_json(&data)
                            .await.unwrap();

    let body = response.body().await.unwrap();
    
    HttpResponse::Ok().body(body)
}