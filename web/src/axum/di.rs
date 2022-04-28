use axum::Extension;
use common::uuid::DefaultUuid;
use db::memdb::MemdbWrapper;
use gateways::todo::TaskGateway;

trait Provider {
    fn provide<T: Clone + Send + Sync + 'static>(self, obj: T) -> axum::Router;
}

impl Provider for axum::Router {
    fn provide<T: Clone + Send + Sync + 'static>(self, obj: T) -> axum::Router {
        self.layer(Extension(obj))
    }
}

pub async fn provide(app: axum::Router) -> axum::Router {
    let kvs = MemdbWrapper::new_dyn().await;
    let uuid_util = DefaultUuid::new_dyn();
    app.provide(TaskGateway::new_dyn(kvs, uuid_util))
        .provide(MemdbWrapper::new_dyn().await)
}
