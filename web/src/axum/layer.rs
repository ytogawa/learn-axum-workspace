use axum::http::Request;
use common::uuid::{DefaultUuid, UuidUtil};
use tower_http::{
    request_id::{MakeRequestId, RequestId, SetRequestIdLayer},
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Default)]
struct UuidMakeRequestId {}

impl MakeRequestId for UuidMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        DefaultUuid::new()
            .gen()
            .to_string()
            .parse()
            .map(|x| RequestId::new(x))
            .ok()
    }
}

pub fn layer(app: axum::Router) -> axum::Router {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "web=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    app.layer(
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true)),
    )
    // x-request-idの設定は必ず最初に実行されるようにしたいので、最後に設定する
    .layer(SetRequestIdLayer::x_request_id(UuidMakeRequestId {}))
}
