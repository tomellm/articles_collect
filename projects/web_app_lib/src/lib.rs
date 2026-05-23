mod app;
mod articles;
mod keycloak;
mod routes;
#[cfg(feature = "ssr")]
mod server_router;
#[cfg(feature = "ssr")]
mod state;
mod tags;
mod utils;

#[cfg(feature = "ssr")]
pub async fn server_start() {
    use leptos::prelude::*;
    use tracing::{info, warn};
    use tracing_subscriber::prelude::*;

    use crate::server_router::router;

    if let Err(err) = dotenv::dotenv() {
        warn!("loading .env file failed: {err}");
    }

    // setup logging for different libs
    let log_filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("tokio", tracing::Level::WARN)
        .with_target("runtime", tracing::Level::WARN)
        .with_target("sqlx::query", tracing::Level::WARN);

    // setup formatting for layers
    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_thread_names(false)
        .with_thread_ids(false);

    let fmt_layer_filtered = fmt_layer.with_filter(log_filter);

    tracing_subscriber::Registry::default()
        .with(fmt_layer_filtered)
        .init();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let app_service = router(leptos_options).await;

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app_service).await.unwrap();
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::TRACE)
            .build(),
    );

    leptos::mount::hydrate_body(App);
}
