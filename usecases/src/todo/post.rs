use common::{error::Error, uuid::DynUuidUtil};
use domains::todo::task::{DynTaskRepository, Task};

pub struct TaskPost {
    pub title: String,
    pub content: String,
}

pub async fn execute(
    post: TaskPost,
    repo: DynTaskRepository,
    uuid_util: DynUuidUtil,
) -> Result<Task, Error> {
    let task = Task::create(post.title, post.content, uuid_util);
    repo.store(task).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::uuid::DefaultUuid;
    use domains::todo::task::{Task, TaskId, TaskRepository};
    use std::sync::Arc;

    mockall::mock! {
        Repository {}
        #[common::asynchronous::async_trait]
        impl TaskRepository for Repository {
            async fn retrieve(&self, id: TaskId) -> Result<Task, Error>;
            async fn store(&self, task: Task) -> Result<Task, Error>;
        }
    }

    #[tokio::test]
    async fn it_works() {
        let post = TaskPost {
            title: "title".to_string(),
            content: "content".to_string(),
        };

        let mut repo = MockRepository::new();
        repo.expect_store().times(1).returning(|task| Ok(task));

        let default_uuid = DefaultUuid {};

        let task = execute(post, Arc::new(repo), Arc::new(default_uuid)).await.unwrap();
        assert!(!task.id.value().is_nil());
        assert_eq!(task.title, "title");
        assert_eq!(task.content, "content");
    }
}
