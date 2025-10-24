use std::{collections::HashMap, sync::Arc};

use actix_web::{HttpResponse, Responder, put, web};
use anyhow::Context;
use serde::Deserialize;
use tokio::{process::Command, sync::RwLock};
use utoipa_actix_web::service_config::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(execute_task);
}

#[utoipa::path(
    operation_id = "executeTask",
    responses(
        (status = 200, description = "Task executed successfully"),
        (status = 404, description = "Endpoint not found"),
        (status = 500, description = "Internal server error during task execution"),
    ),
    params(
        ("endpoint" = String, Path, description = "The name of the task to execute")
    ),
)]
#[put("/{endpoint}")]
async fn execute_task(
    endpoint: web::Path<String>,
    task_router: web::Data<Arc<RwLock<TaskRouter>>>,
) -> actix_web::Result<impl Responder> {
    let endpoint = endpoint.to_string();

    let task_router = task_router.into_inner();
    let task_router = task_router.read().await;

    let task = task_router
        .get(&endpoint)
        .context(format!("Endpoint '{}' not found", &endpoint));
    let task = match task {
        Ok(v) => v,
        Err(e) => {
            log::error!("{}", e);
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    let output = task.execute().await.context("Failed to execute the task");
    let output = match output {
        Ok(v) => v,
        Err(e) => {
            log::error!("task '{}': {}", &endpoint, e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    Ok(HttpResponse::Ok().content_type("text/plain").body(output))
}

#[derive(Deserialize)]
pub struct Task(String);

impl TryFrom<&Task> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &Task) -> Result<Self, Self::Error> {
        let parts: Vec<String> = value.0.split_whitespace().map(|e| e.to_string()).collect();

        let command = parts.get(0).context("No command found")?.to_string();
        let args = parts[1..].to_vec();

        let mut cmd = Command::new(command);
        cmd.args(args);

        Ok(cmd)
    }
}

impl Task {
    async fn execute(&self) -> anyhow::Result<String> {
        let mut cmd = Command::try_from(self).context("Failed to parse the command")?;
        let output = cmd.output().await.context("Command failed to run")?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(stdout)
    }
}

pub type TaskRouter = HashMap<String, Task>;
