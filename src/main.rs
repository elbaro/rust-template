use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Debug, Parser)]
struct Args {
    #[clap(long, default_value = "0.0.0.0:3000")]
    bind: std::net::SocketAddr,
}

// fn install_tracing() -> Result<()> {
//     use opentelemetry::sdk::trace::Sampler;
//     use opentelemetry_otlp::WithExportConfig;
//     use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
//     use tracing_subscriber::Registry;

//     let tracer = opentelemetry_otlp::new_pipeline()
//         .tracing()
//         .with_exporter(
//             opentelemetry_otlp::new_exporter()
//                 .http()
//                 .with_endpoint("http://tracing1:4317")
//                 .with_timeout(std::time::Duration::from_secs(3)),
//         )
//         .with_trace_config(
//             opentelemetry::sdk::trace::config()
//                 .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
//                     1.0,
//                 ))))
//                 .with_resource(opentelemetry::sdk::Resource::new(vec![
//                     opentelemetry::KeyValue::new("service.name", env!("CARGO_BIN_NAME")),
//                 ])),
//         )
//         .install_batch(opentelemetry::runtime::Tokio)?;
//     let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
//     let subscriber = Registry::default()
//         .with(tracing_subscriber::fmt::layer())
//         .with(telemetry);
//     tracing::subscriber::set_global_default(subscriber)?;

//     Ok(())
// }

async fn serve(bind_addr: SocketAddr) {
    let app = Router::new()
        .route("/", get(index))
        .route("/aa", get(aa))
        .with_state(Arc::new(0));

    axum::Server::bind(&bind_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("data store disconnected {}", 123)]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("Parse Failed")]
    InvalidInput {
        #[from]
        source: SerdeError,
        backtrace: Backtrace,
    }
    #[error("unknown data store error")]
    Unknown,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    pretty_env_logger::init_timed();
    color_eyre::install()?; // panic handler
    install_tracing()?;

    let args = Args::parse();
    log::info!("{:#?}", args);

    Ok(())
}
