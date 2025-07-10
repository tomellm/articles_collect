use std::env;

use leptos::prelude::*;
use leptos_keycloak_auth::{
    components::{EndSession, ShowWhenAuthenticated},
    expect_keycloak_auth, init_keycloak_auth, to_current_url,
    url::Url,
    UseKeycloakAuthOptions, ValidationOptions,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakInfo {
    pub url: String,
    pub realm_name: String,
    pub client_id: String,
}

impl KeycloakInfo {
    #[cfg(feature = "ssr")]
    pub fn from_env() -> Self {
        let url = env::var("KEYCLOAK_URL").expect("env KEYCLOAK_URL not found");
        let realm_name =
            env::var("KEYCLOAK_REALM_NAME").expect("env KEYCLOAK_REALM_NAME not found");
        let client_id = env::var("KEYCLOAK_CLIENT_ID").expect("env KEYCLOAK_CLIENT_ID not found");
        Self {
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
    let auth = expect_keycloak_auth();
    //let login_url_unavailable = Signal::derive(move || auth.login_url.get().is_none());
    let login_url = Signal::derive(move || {
        auth.login_url
            .get()
            .map(|url| url.to_string())
            .unwrap_or_default()
    });
    view! {
        <a class="text-xl"
            href=move || login_url.get()>
            "Log in"
        </a>
    }
}

#[component]
pub fn InitAuth(children: ChildrenFn) -> impl IntoView {
    let keycloak_info = LocalResource::new(|| async move { get_keycloak_info().await.unwrap() });
    view! {
        <Suspense fallback=|| view! { "" }>
            {Suspend::new(async move {
                let info = keycloak_info.await;
                provide_context(info.clone());
                let _auth = init_keycloak_auth(UseKeycloakAuthOptions {
                    keycloak_server_url: Url::parse(&info.url).unwrap(),
                    realm: info.realm_name.clone(),
                    client_id: info.client_id.clone(),
                    post_login_redirect_url: to_current_url(),
                    post_logout_redirect_url: to_current_url(),
                    scope: vec![],
                    id_token_validation: ValidationOptions {
                        expected_audiences: Some(vec![info.client_id]),
                        expected_issuers: Some(vec![format!("{}/realms/{}", info.url, info.realm_name)]),
                    },
                    delay_during_hydration: false,
                    advanced: Default::default(),
                });
                { children() }
            })}
        </Suspense>
    }
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
    view! {
        <Protected>
            <EndSession and_route_to="http://localhost:3000"/>
        </Protected>
    }
}
#[component]
pub fn UserNavBar() -> impl IntoView {
    let auth = expect_keycloak_auth();
    view! {
        <button on:click=move |_| auth.end_session()>
            "Logout"
        </button>
    }
}
