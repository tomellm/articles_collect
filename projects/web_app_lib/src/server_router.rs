use std::{env, sync::Arc, time::Duration};

use crate::{
    app::{App, shell},
    keycloak::KeycloakInfo,
    state::ServerState,
};
use axum::{
    BoxError, Router,
    error_handling::HandleErrorLayer,
    extract::{Path, State},
    http::{Request, StatusCode, Uri},
    response::IntoResponse,
    routing::{IntoMakeService, get},
};
use axum_keycloak_auth::{
    NonEmpty, PassthroughMode, Url,
    extract::{AuthHeaderTokenExtractor, TokenExtractor},
    instance::KeycloakConfig,
};
use axum_keycloak_auth::{
    decode::ProfileAndEmail, instance::KeycloakAuthInstance, layer::KeycloakAuthLayer,
};
use leptos::{config::LeptosOptions, error::Errors, prelude::*, view};
use leptos_axum::{LeptosRoutes, generate_route_list, handle_server_fns_with_context};
use sea_orm::{DatabaseConnection, DbErr};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{
    services::ServeDir,
    trace::{DefaultOnFailure, DefaultOnRequest, TraceLayer},
};

pub async fn file_and_error_handler(
    uri: Uri,
    State(state): State<ServerState>,
    req: Request<axum::body::Body>,
) -> axum::response::Response {
    let root = state.leptos_options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        tracing::warn!("{:?}:{}", res.status(), uri);
        let handler =
            leptos_axum::render_app_to_stream(|| error_template(RwSignal::new(Errors::default())));
        handler(req).await.into_response()
    }
}

async fn get_static_file(
    uri: Uri,
    root: &str,
) -> Result<axum::response::Response<axum::body::Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(axum::body::Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

pub fn error_template(errors: RwSignal<Errors>) -> impl IntoView {
    view! {
      <h1>"Errors"</h1>
      <For
          // a function that returns the items we're iterating over; a signal is fine
          each=move || errors.get()
          // a unique key for each item as a reference
          key=|(key, _)| key.clone()
          // renders each item to a view
          children= move | (_, error)| {
          let error_string = error.to_string();
            view! {
              <p>"Error: " {error_string}</p>
            }
          }
      />
    }
    .into_view()
}

async fn server_fn_handler(
    State(server_state): State<ServerState>,
    path: Path<String>,
    request: Request<axum::body::Body>,
) -> impl IntoResponse {
    tracing::trace!("Request to: '{:?}'", path);

    handle_server_fns_with_context(move || server_state.register_all_to_context(), request).await
}

pub async fn leptos_routes_handler(
    State(server_state): State<ServerState>,
    request: Request<axum::body::Body>,
) -> axum::response::Response {
    let options = server_state.leptos_options.clone();

    let handler = leptos_axum::render_app_async_with_context(
        move || server_state.register_all_to_context(),
        move || shell(options.clone()),
    );

    handler(request).await.into_response()
}

/// Creates the axum router that will then handle all of the requests on the
/// backend. This also contains things like public and private routes as well
/// as error handeling and authentication.
///
/// # Panics
///
/// Panics if something doesnt work on startup since there is not reason to do
/// a clean error return, the service should just crash
pub async fn router(leptos_options: LeptosOptions) -> IntoMakeService<Router> {
    let connection = setup_database().await.unwrap();
    let state = ServerState::new(connection, leptos_options);

    let client_router = Router::new()
        .leptos_routes_with_handler(generate_route_list(App), get(leptos_routes_handler));

    let server_router = Router::new()
        .route(
            "/api/{*fn_name}",
            get(server_fn_handler).post(server_fn_handler),
        )
        .layer(setup_keycloak())
        .route(
            "/public/api/{*fn_name}",
            get(server_fn_handler).post(server_fn_handler),
        );

    Router::new()
        .merge(client_router)
        .merge(server_router)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_keycloak_auth_error))
                .timeout(Duration::from_secs(30)),
        )
        .fallback(file_and_error_handler)
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::default())
                .on_failure(DefaultOnFailure::default()),
        )
        .into_make_service()
}

async fn setup_database() -> Result<DatabaseConnection, DbErr> {
    use std::env;
    // use migration::{Migrator, MigratorTrait};

    let mut conn_opt = sea_orm::ConnectOptions::new(env::var("DATABASE_URL").unwrap());
    conn_opt
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Info);

    let connection = sea_orm::Database::connect(conn_opt).await.unwrap();

    // let pending_migrations = Migrator::get_pending_migrations(&connection).await?;
    // Migrator::up(&connection, Some(pending_migrations.len() as u32)).await?;

    Ok(connection)
}

fn setup_keycloak() -> KeycloakAuthLayer<String, ProfileAndEmail> {
    let keycloak_info = KeycloakInfo::from_env();

    let instance = KeycloakAuthInstance::new(
        KeycloakConfig::builder()
            .server(Url::parse(keycloak_info.url.as_str()).unwrap())
            .realm(keycloak_info.realm_name)
            .build(),
    );

    KeycloakAuthLayer::<String>::builder()
        .instance(instance)
        .passthrough_mode(PassthroughMode::Block)
        .persist_raw_claims(false)
        .expected_audiences(vec![env::var("KEYCLOAK_CLIENT_ID").unwrap()])
        .token_extractors(NonEmpty::<Arc<dyn TokenExtractor>> {
            head: Arc::new(AuthHeaderTokenExtractor::default()),
            tail: vec![],
        })
        .build()
}

async fn handle_keycloak_auth_error(
    // `Method` and `Uri` are extractors so they can be used here
    // the last argument must be the error itself
    err: BoxError,
) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("keycloak auth failed with {err}"),
        // cant be used because I would need the other parameters, method uri
        // format!("keycloak auth `{method} {uri}` failed with {err}"),
    )
}
