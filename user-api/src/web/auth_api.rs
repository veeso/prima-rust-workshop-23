use super::{SessionClient, WebserverData};
use crate::proto::{
    store_client::types::{AuthError, AuthResponse as StoreAuthResponse},
    StoreClient,
};

use actix_session::Session;
use actix_web::{get, post, web, Error, HttpResponse, Result};
use uuid::Uuid;

impl actix_web::error::ResponseError for crate::proto::ProtobufError {}

#[derive(Deserialize, Debug)]
struct SignInRequest {
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct SignUpRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct AuthResponse {
    id: Uuid,
    email: String,
}

#[post("/auth/sign-in")]
async fn sign_in(
    request: web::Json<SignInRequest>,
    data: web::Data<WebserverData>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session = SessionClient::from(session);
    let payload = request.into_inner();
    debug!("sign-in request {:?}", payload);
    // init store
    let mut store_client = StoreClient::connect(data.store_client_url.clone()).await?;
    let sign_in_result = store_client
        .sign_in(&payload.email, &payload.password)
        .await?;
    if let Some(id) = sign_in_result.user_id() {
        // put into session
        session.set_user(&id, &payload.email);
        Ok(HttpResponse::Ok().json(AuthResponse {
            id,
            email: payload.email,
        }))
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

#[post("/auth/sign-up")]
async fn sign_up(
    request: web::Json<SignUpRequest>,
    data: web::Data<WebserverData>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session = SessionClient::from(session);
    let payload = request.into_inner();
    debug!("sign-up request {:?}", payload);
    // init store
    let mut store_client = StoreClient::connect(data.store_client_url.clone()).await?;
    let sign_up_result = store_client
        .sign_up(&payload.email, &payload.password)
        .await?;
    match sign_up_result {
        StoreAuthResponse::Authenticated(id) => {
            // put into session
            session.set_user(&id, &payload.email);
            Ok(HttpResponse::Ok().json(AuthResponse {
                id,
                email: payload.email,
            }))
        }
        StoreAuthResponse::Failed(AuthError::EmailAlreadyTaken) => {
            Ok(HttpResponse::Conflict().finish())
        }
        StoreAuthResponse::Failed(_) => Ok(HttpResponse::BadRequest().finish()),
    }
}

#[get("/auth")]
async fn auth(session: Session) -> Result<HttpResponse, Error> {
    let session = SessionClient::from(session);
    debug!("auth request");
    if let Some(user) = session.get_user() {
        Ok(HttpResponse::Ok().json(AuthResponse {
            id: user.id,
            email: user.email,
        }))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}
