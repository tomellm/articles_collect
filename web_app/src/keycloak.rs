use std::{env, str::FromStr};

use leptos::prelude::*;
use leptos_keycloak_auth::{
    components::{EndSession, ShowWhenAuthenticated},
    expect_keycloak_auth, init_keycloak_auth, to_current_url,
    url::Url,
    UseKeycloakAuthOptions, ValidationOptions,
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakInfo {
    pub app_url: Url,
    pub url: String,
    pub realm_name: String,
    pub client_id: String,
}

impl KeycloakInfo {
    pub fn from_env() -> Self {
        let app_url = Url::from_str(&env::var("APP_URL").expect("env APP_URL not found")).unwrap();
        let url = env::var("KEYCLOAK_URL").expect("env KEYCLOAK_URL not found");
        let realm_name =
            env::var("KEYCLOAK_REALM_NAME").expect("env KEYCLOAK_REALM_NAME not found");
        let client_id = env::var("KEYCLOAK_CLIENT_ID").expect("env KEYCLOAK_CLIENT_ID not found");
        Self {
            app_url,
            url,
            realm_name,
            client_id,
        }
    }
}

#[server]
async fn get_keycloak_info() -> Result<KeycloakInfo, ServerFnError> {
    Ok(KeycloakInfo::from_env())
}

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <h1 id="unauthenticated">"Unauthenticated"</h1>
        <LoginButton />
    }
}

#[component]
pub fn LoginButton() -> impl IntoView {
    info!("rendering login button");
    let auth = expect_keycloak_auth();
    let login_url_unavailable = Signal::derive(move || auth.login_url.get().is_none());
    let login_url = Signal::derive(move || {
        auth.login_url
            .get()
            .map(|url| url.to_string())
            .unwrap_or_default()
    });
    view! {
        <a href=move || login_url.get() class="text-xl"
            aria_disabled=login_url_unavailable>
            "Log in"
        </a>
    }
}

#[component]
pub fn InitAuth(children: ChildrenFn) -> impl IntoView {
    let info = SharedValue::new(KeycloakInfo::from_env).into_inner();
    let _auth = init_keycloak_auth(UseKeycloakAuthOptions {
        keycloak_server_url: Url::parse(&info.url).unwrap(),
        realm: info.realm_name.clone(),
        client_id: info.client_id.clone(),
        post_login_redirect_url: info.app_url.clone(),
        post_logout_redirect_url: info.app_url.clone(),
        scope: vec![],
        id_token_validation: ValidationOptions {
            expected_audiences: Some(vec![info.client_id]),
            expected_issuers: Some(vec![format!("{}/realms/{}", info.url, info.realm_name)]),
        },
        delay_during_hydration: true,
        advanced: Default::default(),
    });
    children()
}

#[component]
pub fn Protected(children: ChildrenFn) -> impl IntoView {
    view! {
        <ShowWhenAuthenticated fallback=|| view! { <Login/> }>
            { children() }
        </ShowWhenAuthenticated>
    }
    //<DebugState/>
}

#[component]
pub fn Logout() -> impl IntoView {
    let auth = expect_keycloak_auth();
    view! {
        <button on:click=move |_| auth.end_session()
            class="text-xl">
            "Logout"
        </button>
    }
}
