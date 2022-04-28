use common::{asynchronous::async_trait, error::Error};
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait Kvs {
    async fn get(&self, key: String) -> Result<String, Error>;
    async fn set(&mut self, key: String, value: String) -> Result<String, Error>;
    async fn delete(&mut self, key: String) -> Result<String, Error>;
}

pub type DynKvs = Arc<RwLock<dyn Kvs + Send + Sync>>;
