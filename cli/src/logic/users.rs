use anyhow::{Context, Result};
use dialoguer::{Input, Password};
use reqwest::{Client, Response, header::SET_COOKIE};

use crate::{auth, http::users as user_api};

pub struct UserState<'a> {
    client: &'a Client,
}

impl<'a> UserState<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn login(&self) -> Result<()> {
        let email: String = Input::new().with_prompt("Email").interact_text()?;

        let password = Password::new().with_prompt("Password").interact()?;

        let response = user_api::login(self.client, &user_api::LoginRequest { email, password })
            .await?
            .error_for_status()?;

        auth::store_auth_creds(
            extract_cookie(&response, "sessionid")?.as_str(),
            extract_cookie(&response, "csrftoken")?.as_str(),
        )?;

        println!("Logged in.");
        Ok(())
    }

    pub async fn signup(&self) -> Result<()> {
        let email: String = Input::new().with_prompt("Email").interact_text()?;
        let username: String = Input::new().with_prompt("Username").interact_text()?;

        let password = loop {
            let password = Password::new().with_prompt("Password").interact()?;

            let repeat_password = Password::new().with_prompt("Repeat password").interact()?;

            if password == repeat_password {
                break password;
            }

            eprintln!("Passwords do not match. Try again.");
        };

        user_api::signup(
            self.client,
            &user_api::SignupRequest {
                email,
                password,
                username,
            },
        )
        .await?;

        println!("Account created. Now log in: jot login.");
        Ok(())
    }

    pub async fn logout(&self) -> Result<()> {
        user_api::logout(self.client).await?.error_for_status()?;
        auth::deauthenticate_client()?;

        println!("Logged out.");
        Ok(())
    }
}

fn extract_cookie(response: &Response, name: &str) -> Result<String> {
    let prefix = format!("{name}=");

    response
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|header| header.to_str().ok())
        .map(|header| header.split(';').next().unwrap_or_default())
        .find_map(|cookie| cookie.strip_prefix(&prefix).map(str::to_owned))
        .with_context(|| format!("Login response did not contain a {name} cookie"))
}
