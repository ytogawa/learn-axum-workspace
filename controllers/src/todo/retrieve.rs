use axum::{extract::Path, Extension, Json};
use common::uuid::Uuid;
use domains::todo::task::{DynTaskRepository, TaskId};
use serde::{Deserialize, Serialize};
use usecases::todo::retrieve::execute;

use crate::error::AppError;

#[derive(Deserialize, Debug)]
pub struct RetrieveReq {
    id: Uuid,
}

#[derive(Serialize, Debug)]
pub struct RetrieveRes {
    id: String,
    title: String,
    content: String,
}

pub async fn handle(
    Path(req): Path<RetrieveReq>,
    repo: Extension<DynTaskRepository>,
) -> Result<Json<RetrieveRes>, AppError> {
    let id = TaskId::new(req.id);
    let repo = (*repo).clone();
    let task = execute(id, repo).await?;
    Ok(Json(RetrieveRes {
        id: task.id.value().to_string(),
        title: task.title,
        content: task.content,
    }))
}
