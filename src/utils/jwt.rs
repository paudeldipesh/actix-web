use super::constants;
use chrono::{Duration, Utc};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32,
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, Error> {
    let now: chrono::DateTime<Utc> = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);

    let claims: Claims = Claims {
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
    };

    let secret: String = (*constants::SECRET).clone();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, Error> {
    let secret: String = (*constants::SECRET).clone();
    let claim_data: Result<TokenData<Claims>, Error> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );
    claim_data
}
