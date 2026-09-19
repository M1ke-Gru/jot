use std::io;

use chrono::Utc;
use reqwest::Client;
use reqwest::Error as ReqError;

use crate::http::tasks::TaskUpdate;
use crate::http::tasks::{self as tasks_router, TaskFull};
use crate::task::task_schema::TaskCreate;

pub struct TaskState<'a> {
    client: &'a Client,
}

impl<'a> TaskState<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn add(&self, task: TaskCreate) -> Result<(), ReqError> {
        let t = tasks_router::TaskCreate {
            title: task.name,
            description: "".into(),
            status: false,
            created_at: Utc::now().to_rfc3339(),
        };
        tasks_router::create_task(self.client, &t).await?;
        Ok(())
    }

    pub async fn rm(&self, name: String) -> Result<(), ReqError> {
        if let Some(task) = self.find_task_by_name(name).await? {
            tasks_router::delete_task(self.client, task.id).await?;
            println!("Removed task: {}", task.title);
        }
        Ok(())
    }

    pub async fn mark_done(&self, name: String) -> Result<(), ReqError> {
        if let Some(task) = self.find_task_by_name(name).await? {
            tasks_router::update_task(
                self.client,
                task.id,
                &TaskUpdate {
                    title: None,
                    status: Some(!(task.status)),
                    description: None,
                },
            )
            .await?;
            println!("Done: {}", task.title);
        }
        Ok(())
    }

    pub async fn ls(&self) -> Result<(), ReqError> {
        let array = tasks_router::list_tasks(self.client).await?;
        for (i, t) in array.iter().enumerate() {
            let checkmark = if t.status { "x" } else { " " };
            println!("{}. [{}] {}", i + 1, checkmark, t.title);
        }
        Ok(())
    }

    async fn find_task_by_name(&self, name: String) -> Result<Option<TaskFull>, ReqError> {
        let tasks = tasks_router::list_tasks(self.client).await?;
        let mut filtered_tasks: Vec<TaskFull> = tasks
            .into_iter()
            .filter(|t| t.title.contains(name.as_str()))
            .collect::<Vec<TaskFull>>();

        if filtered_tasks.len() == 1 {
            return Ok(filtered_tasks.pop());
        }

        for (choice, task) in filtered_tasks.iter().enumerate() {
            println!("{}. {}", choice + 1, task.title);
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Ok(choice) if choice >= 1 && choice <= filtered_tasks.len() => {
                Ok(Some(filtered_tasks.remove(choice - 1)))
            }
            _ => {
                println!("Provide a valid integer.");
                Ok(None)
            }
        }
    }
}
