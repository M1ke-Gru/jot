use anyhow::Result;
use keyring_core::Entry;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, COOKIE},
    Client,
};

const KEYRING_SERVICE: &str = "jot";
const SESSION_COOKIE: &str = "sessionid";
const CSRF_COOKIE: &str = "csrftoken";

pub fn store_auth_creds(sessionid: &str, csrftoken: &str) -> Result<()> {
    Entry::new(KEYRING_SERVICE, SESSION_COOKIE)?.set_password(sessionid)?;
    Entry::new(KEYRING_SERVICE, CSRF_COOKIE)?.set_password(csrftoken)?;
    Ok(())
}

pub fn authenticated_client() -> Result<Client> {
    let sessionid = Entry::new(KEYRING_SERVICE, SESSION_COOKIE)?.get_password()?;
    let csrftoken = Entry::new(KEYRING_SERVICE, CSRF_COOKIE)?.get_password()?;

    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&format!("sessionid={sessionid}; csrftoken={csrftoken}"))?,
    );
    headers.insert(
        HeaderName::from_static("x-csrftoken"),
        HeaderValue::from_str(&csrftoken)?,
    );

    Ok(Client::builder().default_headers(headers).build()?)
}

pub fn deauthenticate_client() -> Result<()> {
    Entry::new(KEYRING_SERVICE, SESSION_COOKIE)?.delete_credential()?;
    Entry::new(KEYRING_SERVICE, CSRF_COOKIE)?.delete_credential()?;

    Ok(())
}
