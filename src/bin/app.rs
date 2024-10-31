use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use adapter::database::connect_database_with;
use anyhow::Result;
#[cfg(debug_assertions)]
use api::openapi::ApiDoc;
use api::route::v1;

use axum::{http::Method, Router};
use opentelemetry::global;
use registry::AppRegistryImpl;
use shared::config::AppConfig;
use tokio::net::TcpListener;

use shared::env::{which, Environment};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use anyhow::Context;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;

use tower_http::cors::{self, CorsLayer};
#[cfg(debug_assertions)]
use utoipa::OpenApi;
#[cfg(debug_assertions)]
use utoipa_redoc::{Redoc, Servable};

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(cors::Any)
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    let host = std::env::var("JAEGER_HOST")?;
    let port = std::env::var("JAEGER_PORT")?;
    let endpoint = format!("{host}:{port}");

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(endpoint)
        .with_service_name("rust-web-base")
        .with_auto_split_batch(true)
        .with_max_packet_size(8192)
        .install_simple()?;

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    #[cfg(not(debug_assertions))]
    let subscriber = subscriber.json();

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .with(opentelemetry)
        .try_init()?;

    Ok(())
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);

    let registry = Arc::new(AppRegistryImpl::new(pool, app_config));

    let router = Router::new().merge(v1::routes());
    #[cfg(debug_assertions)]
    let router = router.merge(Redoc::with_url("/docs", ApiDoc::openapi()));

    let app = router
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .layer(cors())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Unexpected error happened in server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,error.message = %e, "Unexpected error"
            )
        })
}

async fn shutdown_signal() {
    fn purge_spans() {
        global::shutdown_tracer_provider();
    }

    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signall handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await
            .expect("Failed to receive SIGTERM signal");
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl-Cを受信しました.");
            purge_spans();
        },
        _ = terminate => {
            tracing::info!("SIGTERMを受信しました.");
            purge_spans();
        }
    }
}
