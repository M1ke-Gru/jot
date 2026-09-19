use super::create_url;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const URL_PART: &str = "/tasks";

#[derive(Serialize, Debug)]
pub struct TaskCreate {
    pub title: String,
    pub status: bool,
    pub description: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskFull {
    pub id: Uuid,
    pub title: String,
    pub status: bool,
    pub description: String,
    pub created_at: String,
}

#[derive(Serialize, Debug)]
pub struct TaskUpdate {
    pub title: Option<String>,
    pub status: Option<bool>,
    pub description: Option<String>,
}

pub async fn list_tasks(client: &reqwest::Client) -> Result<Vec<TaskFull>, reqwest::Error> {
    client
        .get(create_url(URL_PART, "/"))
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<TaskFull>>()
        .await
}

pub async fn create_task(
    client: &reqwest::Client,
    task: &TaskCreate,
) -> Result<TaskFull, reqwest::Error> {
    client
        .post(create_url(URL_PART, "/"))
        .json(task)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}

pub async fn update_task(
    client: &reqwest::Client,
    task_id: Uuid,
    task: &TaskUpdate,
) -> Result<TaskFull, reqwest::Error> {
    client
        .post(create_url(URL_PART, format!("/update/{task_id}").as_str()))
        .json(task)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}

pub async fn delete_task(
    client: &reqwest::Client,
    task_id: Uuid,
) -> Result<TaskFull, reqwest::Error> {
    client
        .delete(create_url(URL_PART, format!("/update/{task_id}").as_str()))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}
