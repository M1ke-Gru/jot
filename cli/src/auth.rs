use anyhow::Result;
use keyring::Entry;
use reqwest::{
    Client,
    header::{COOKIE, HeaderMap, HeaderName, HeaderValue},
};

const KEYRING_SERVICE: &str = "jot";
const SESSION_COOKIE: &str = "sessionid";
const CSRF_COOKIE: &str = "csrftoken";

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
