use common::{
    asynchronous::async_trait,
    error::Error,
    uuid::{self, DynUuidUtil},
};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskId {
    value: uuid::Uuid,
}

impl TaskId {
    pub fn new(id: uuid::Uuid) -> TaskId {
        TaskId { value: id }
    }

    pub fn value(&self) -> uuid::Uuid {
        self.value.clone()
    }
}

impl From<uuid::Uuid> for TaskId {
    fn from(id: uuid::Uuid) -> Self {
        TaskId { value: id }
    }
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub content: String,
}

impl Task {
    pub fn create(title: String, content: String, uuid_util: DynUuidUtil) -> Task {
        Task {
            id: TaskId::new(uuid_util.gen()),
            title,
            content,
        }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TaskRepository {
    async fn retrieve(&self, id: TaskId) -> Result<Task, Error>;
    async fn store(&self, task: Task) -> Result<Task, Error>;
}

pub type DynTaskRepository = Arc<dyn TaskRepository + Send + Sync>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let title = "title";
        let content = "content";
        let default_uuid = uuid::DefaultUuid::new();
        let task = Task::create(
            title.to_string(),
            content.to_string(),
            Arc::new(default_uuid),
        );
        assert!(!task.id.value.is_nil());
        assert_eq!(task.title, title);
        assert_eq!(task.content, content);
    }
}
