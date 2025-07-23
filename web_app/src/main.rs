#![allow(non_snake_case)]

#[cfg(feature = "ssr")]
use axum_keycloak_auth::{
    decode::ProfileAndEmail, instance::KeycloakAuthInstance, layer::KeycloakAuthLayer,
};
#[cfg(feature = "ssr")]
use sea_orm::{DatabaseConnection, DbErr};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_keycloak_auth::to_current_url;
    use tracing::{info, warn};
    use tracing_subscriber::prelude::*;
    use web_app::{app::*, ServerState};

    if let Err(err) = dotenv::dotenv() {
        warn!("loading .env file failed: {err}");
    }

    let log_filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("tokio", tracing::Level::WARN)
        .with_target("runtime", tracing::Level::WARN)
        .with_target("sqlx::query", tracing::Level::WARN);

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

    let connection = setup_database().await.unwrap();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let state = ServerState::new(connection);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(state.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .layer(setup_keycloak())
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
async fn setup_database() -> Result<DatabaseConnection, DbErr> {
    use std::env;

    use migration::{Migrator, MigratorTrait};
    let connection = sea_orm::Database::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let pending_migrations = Migrator::get_pending_migrations(&connection).await?;
    Migrator::up(&connection, Some(pending_migrations.len() as u32)).await?;

    Ok(connection)
}

#[cfg(feature = "ssr")]
fn setup_keycloak() -> KeycloakAuthLayer<String, ProfileAndEmail> {
    use axum_keycloak_auth::{
        instance::KeycloakConfig, layer::KeycloakAuthLayer, PassthroughMode, Url,
    };
    use web_app::keycloak::KeycloakInfo;

    let keycloak_info = KeycloakInfo::from_env();

    let instance = KeycloakAuthInstance::new(
        KeycloakConfig::builder()
            .server(Url::parse(keycloak_info.url.as_str()).unwrap())
            .realm(keycloak_info.realm_name)
            .build(),
    );

    KeycloakAuthLayer::<String>::builder()
        .instance(instance)
        .passthrough_mode(PassthroughMode::Pass)
        .persist_raw_claims(false)
        .expected_audiences(vec![])
        .build()
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
