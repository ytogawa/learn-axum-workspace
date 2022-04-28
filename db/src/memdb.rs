use common::{asynchronous::async_trait, error::Error};
use gateways::kvs::{DynKvs, Kvs};
use memdb::Memdb;
use std::{backtrace::Backtrace, sync::Arc};
use tokio::sync::RwLock;

pub struct MemdbWrapper {
    db: Arc<RwLock<Memdb>>,
}

impl MemdbWrapper {
    async fn new() -> Self {
        let db = Memdb::open().await.unwrap();
        MemdbWrapper {
            db: Arc::new(RwLock::new(db)),
        }
    }

    pub async fn new_dyn() -> DynKvs {
        let kvs = Self::new().await;
        Arc::new(RwLock::new(kvs))
    }
}

#[async_trait]
impl Kvs for MemdbWrapper {
    async fn get(&self, key: String) -> Result<String, Error> {
        let lock = self.db.read().await;
        let result = lock.get(key.as_bytes()).await?;
        match result {
            None => Err(Error::NotFound(None, Backtrace::capture())),
            Some(bytes) => match String::from_utf8(bytes) {
                Ok(utf8) => Ok(utf8),
                Err(err) => Err(Error::from(err)),
            },
        }
    }

    async fn set(&mut self, key: String, value: String) -> Result<String, Error> {
        let mut lock = self.db.write().await;
        let result = lock.set(key.as_bytes(), value.as_bytes()).await?;
        match result {
            None => Ok(value.clone()),
            Some(bytes) => match String::from_utf8(bytes) {
                Ok(utf8) => Ok(utf8),
                Err(err) => Err(Error::from(err)),
            },
        }
    }

    async fn delete(&mut self, key: String) -> Result<String, Error> {
        let mut lock = self.db.write().await;
        let result = lock.del(key.as_bytes()).await?;
        match result {
            None => Err(Error::NotFound(None, Backtrace::capture())),
            Some(bytes) => match String::from_utf8(bytes) {
                Ok(utf8) => Ok(utf8),
                Err(err) => Err(Error::from(err)),
            },
        }
    }
}
