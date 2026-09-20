mod auth;
mod http;
mod logic;
mod task;

use crate::auth::authenticated_client;
use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::Client;

use crate::{
    logic::{tasks::TaskState, users::UserState},
    task::task_schema::TaskCreate,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { text: String },
    Rm { name: String },
    Done { name: String },
    List,
    Login,
    Logout,
    Signup,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    keyring::cli::use_native_store(false)?;

    match cli.cmd {
        Command::Login => {
            let client = Client::new();
            UserState::new(&client).login().await?;
        }
        Command::Signup => {
            let client = Client::new();
            UserState::new(&client).signup().await?;
        }
        Command::Logout => {
            let client = authenticated_client()?;
            UserState::new(&client).logout().await?;
        }
        Command::Add { text } => {
            let client = authenticated_client()?;
            let state = TaskState::new(&client);
            state.add(TaskCreate { name: text }).await?;
        }
        Command::Rm { name } => {
            let client = authenticated_client()?;
            TaskState::new(&client).rm(name).await?;
        }
        Command::Done { name } => {
            let client = authenticated_client()?;
            TaskState::new(&client).mark_done(name).await?;
        }
        Command::List => {
            let client = authenticated_client()?;
            TaskState::new(&client).ls().await?;
        }
    }

    Ok(())
}
