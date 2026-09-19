use reqwest::{Client, Error, Response};

use crate::http::create_url;

const URL_PART: &str = "/users";

#[derive(serde::Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(serde::Deserialize)]
pub struct SignupResponse {
    //    id: String,
    //    email: String,
}

#[derive(serde::Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn signup(client: &Client, req: &SignupRequest) -> Result<SignupResponse, Error> {
    client
        .post(create_url(URL_PART, "/signup"))
        .json(req)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}

pub async fn login(client: &Client, req: &LoginRequest) -> Result<Response, Error> {
    client
        .post(create_url(URL_PART, "/login"))
        .json(req)
        .send()
        .await
}

pub async fn logout(client: &Client) -> Result<Response, Error> {
    client.post(create_url(URL_PART, "/logout")).send().await
}
