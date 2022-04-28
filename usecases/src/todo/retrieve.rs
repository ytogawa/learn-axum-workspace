use common::error::Error;
use domains::todo::task::{DynTaskRepository, Task, TaskId};

pub async fn execute(id: TaskId, repo: DynTaskRepository) -> Result<Task, Error> {
    repo.retrieve(id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::uuid::{DefaultUuid, UuidUtil};
    use domains::todo::task::TaskRepository;
    use std::sync::Arc;
    use tokio;

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
        let default_uuid = DefaultUuid::new();
        let raw_id = default_uuid.gen();
        let id = TaskId::new(raw_id);
        let mut repo = MockRepository::new();
        repo.expect_retrieve().times(1).returning(move |id| {
            Ok(Task {
                id,
                title: "title".to_string(),
                content: "content".to_string(),
            })
        });
        let task = execute(id.clone(), Arc::new(repo))
            .await
            .unwrap();
        assert_eq!(task.id, id);
        assert_eq!(task.title, "title");
        assert_eq!(task.content, "content");
    }
}
