use crate::error::AppError;
use axum::{Extension, Json};
use common::uuid::DynUuidUtil;
use domains::todo::task::DynTaskRepository;
use serde::{Deserialize, Serialize};
use usecases::todo::post::{execute, TaskPost};

#[derive(Deserialize, Debug)]
pub struct PostReq {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct PostRes {
    pub id: String,
    pub title: String,
    pub content: String,
}

pub async fn handle(
    Json(req): Json<PostReq>,
    repo: Extension<DynTaskRepository>,
    uuid_util: Extension<DynUuidUtil>,
) -> Result<Json<PostRes>, AppError> {
    let post = TaskPost {
        title: req.title,
        content: req.content,
    };
    let repo = (*repo).clone();
    let uuid_util = (*uuid_util).clone();
    let task = execute(post, repo, uuid_util).await?;
    Ok(Json(PostRes {
        id: task.id.value().to_string(),
        title: task.title,
        content: task.content,
    }))
}
