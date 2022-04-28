use common::{asynchronous::async_trait, error::Error, uuid::DynUuidUtil};
use domains::todo::task::{DynTaskRepository, Task, TaskId, TaskRepository};
use serde::{Deserialize, Serialize};

use super::kvs::DynKvs;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
struct TaskDto {
    pub id: String,
    pub title: String,
    pub content: String,
}

pub struct TaskGateway {
    kvs: DynKvs,
    uuid_util: DynUuidUtil,
}

impl TaskGateway {
    pub fn new_dyn(kvs: DynKvs, uuid_util: DynUuidUtil) -> DynTaskRepository {
        Arc::new(TaskGateway { kvs, uuid_util })
    }
}

#[async_trait]
impl TaskRepository for TaskGateway {
    async fn retrieve(&self, id: TaskId) -> Result<Task, Error> {
        let lock = self.kvs.read().await;
        let value = lock.get(id.value().to_string()).await?;
        let dto = serde_json::from_str::<TaskDto>(value.as_str())?;
        Ok(Task {
            id: TaskId::new(self.uuid_util.parse(dto.id.as_str())?),
            title: dto.title,
            content: dto.content,
        })
    }

    async fn store(&self, task: Task) -> Result<Task, Error> {
        let dto = TaskDto {
            id: task.id.value().to_string(),
            title: task.title,
            content: task.content,
        };
        let value = serde_json::to_string::<TaskDto>(&dto)?;
        let mut lock = self.kvs.write().await;
        let value = lock.set(dto.id, value).await?;
        let dto = serde_json::from_str::<TaskDto>(value.as_str())?;
        Ok(Task {
            id: TaskId::new(self.uuid_util.parse(dto.id.as_str())?),
            title: dto.title,
            content: dto.content,
        })
    }
}
