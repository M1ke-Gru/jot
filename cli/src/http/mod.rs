pub(crate) mod tasks;
pub(crate) mod users;

const API_URL: &str = "http://localhost:8000/api";

pub(crate) fn create_url(subpath: &str, request: &str) -> String {
    format!("{API_URL}{subpath}{request}")
}
