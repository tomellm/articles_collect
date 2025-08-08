#[cfg(feature = "ssr")]
use axum::extract::FromRef;
use leptos::config::LeptosOptions;
#[cfg(feature = "ssr")]
use sea_orm::DatabaseConnection;

pub mod app;
pub mod articles;
pub mod keycloak;
pub mod routes;
pub mod utils;

#[cfg(feature = "ssr")]
#[derive(FromRef, Debug, Clone)]
pub struct ServerState {
    pub db: DatabaseConnection,
    pub leptos_options: LeptosOptions,
}

#[cfg(feature = "ssr")]
impl ServerState {
    pub fn new(db: DatabaseConnection, leptos_options: LeptosOptions) -> Self {
        Self { db, leptos_options }
    }
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
