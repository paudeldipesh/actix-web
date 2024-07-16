use crate::utils::{api_response::ApiResponse, jwt::decode_jwt};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::{HeaderValue, AUTHORIZATION},
    Error,
};
use actix_web_lab::middleware::Next;

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth: Option<&HeaderValue> = req.headers().get(AUTHORIZATION);

    if auth.is_none() {
        return Err(Error::from(ApiResponse::new(
            401,
            "unauthorized".to_string(),
        )));
    };

    let token: String = auth
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "")
        .to_owned();
    decode_jwt(token).unwrap();

    next.call(req)
        .await
        .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())))
}
