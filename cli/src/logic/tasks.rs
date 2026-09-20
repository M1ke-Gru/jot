use std::io::{self, Write};

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
            println!(
                "{}: {}",
                if !task.status {
                    "Done"
                } else {
                    "Marked not done"
                },
                task.title
            );
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
        let normalized_name = name.to_lowercase();
        let tasks = tasks_router::list_tasks(self.client).await?;
        let mut filtered_tasks: Vec<TaskFull> = tasks
            .into_iter()
            .filter(|t| t.title.to_lowercase().contains(normalized_name.as_str()))
            .collect::<Vec<TaskFull>>();

        match filtered_tasks.len() {
            0 => {
                println!("No tasks with similar names. Nothing marked done.");
                Ok(None)
            }
            1 => Ok(filtered_tasks.pop()),
            _ => {
                println!("Found multiple tasks with similar names:");
                for (choice, task) in filtered_tasks.iter().enumerate() {
                    println!("{}. {}", choice + 1, task.title);
                }
                print!("Enter the number of the task you want to select: ");
                io::stdout().flush().expect("Failed to flush stdout");

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
    }
}
