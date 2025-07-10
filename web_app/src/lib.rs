#[cfg(feature = "ssr")]
use sea_orm::DatabaseConnection;

pub mod app;
pub mod edit;
pub mod keycloak;
pub mod model;
pub mod routes;

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct ServerState {
    db: DatabaseConnection,
}

#[cfg(feature = "ssr")]
impl ServerState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
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
