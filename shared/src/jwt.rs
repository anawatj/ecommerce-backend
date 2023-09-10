use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use jsonwebtoken::{Header, Validation};
use jsonwebtoken::{EncodingKey, DecodingKey};
use rocket::serde::{Serialize,Deserialize};
use chrono::Utc;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket_contrib::json::Json;
use serde_json::Value;
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: Value,
}

#[derive(Debug)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimData {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClaimData {
    type Error = status::Custom<Json<Response>>;
    async fn from_request(req: &'r Request<'_>)-> request::Outcome<Self, status::Custom<Json<Response>>> {
        if let Some(authen_header) = req.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = decode_token(token.to_string()) {
                    
                        return Outcome::Success(token_data.claims);
                    
                } 
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::Unauthorized,
                Json(Response {
                    message: String::from("Invalid token"),
                    data: serde_json::to_value("").unwrap(),
                }),
            ),
        ))
    }
}

pub fn generate_token(email: String) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = ClaimData {
        iat: now,
        exp: now + ONE_WEEK,
        user: email.to_string(),
        login_session: email.to_string(),
    };
    let secret="Hello";
    
    jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}

fn decode_token(token: String) -> Result<TokenData<ClaimData>> {
    let secret="Hello";
    jsonwebtoken::decode::<ClaimData>(&token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
}

