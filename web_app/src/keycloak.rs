use std::env;

use leptos::prelude::*;
use leptos_oidc::{Auth, AuthParameters, AuthSignal, Challenge, LoginLink, LogoutLink};
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeycloakInfo {
    pub app_url: String,
    pub url: String,
    pub realm_name: String,
    pub client_id: String,
}

impl KeycloakInfo {
    pub fn from_env() -> Self {
        let app_url = env::var("APP_URL").expect("env APP_URL not found");
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
    view! {
        <LoginLink class="text-login">Sign in</LoginLink>
    }
}

#[component]
pub fn InitAuth(children: ChildrenFn) -> impl IntoView {
    let info = SharedValue::new(KeycloakInfo::from_env).into_inner();
    let parameters = AuthParameters {
        issuer: format!("{}/realms/{}", info.url, info.realm_name),
        client_id: info.client_id,
        redirect_uri: info.app_url.clone(),
        post_logout_redirect_uri: info.app_url,
        challenge: Challenge::S256,
        scope: None,
        audience: None,
    };

    let auth: AuthSignal = Auth::signal();
    provide_context(auth);

    let loading_auth = Auth::init(parameters);
    provide_context(loading_auth);

    children()
}

#[component]
pub fn AuthErrorPage() -> impl IntoView {
    let auth =
        use_context::<AuthSignal>().expect("AuthErrorContext: RwSignal<AuthStore> not present");
    let error_message = move || auth.get().error().map(|error| format!("{error:?}"));

    view! {
        <h1>Error occurred</h1>
        <p>There was an error in the authentication process!</p>
        { error_message }
    }
}

#[component]
pub fn Logout() -> impl IntoView {
    view! {
        <LogoutLink class="text-logout">Sign out</LogoutLink>
    }
}

#[component]
pub fn ShowWhenAuthenticated(
    children: ChildrenFn,
    #[prop(optional, into)] fallback: ViewFn,
) -> impl IntoView {
    let auth = expect_context::<AuthSignal>();

    view! {
        <Show
            when=move || auth.get().is_authenticated()
            fallback>
            { children() }
        </Show>
    }
}

#[component]
pub fn ExpectAuth(children: ChildrenFn) -> impl IntoView {
    let auth = expect_context::<AuthSignal>();

    view! {
        { move || match auth.get() {
            Auth::Authenticated(_) => children().into_any(),
            Auth::Loading => view!{
                <div class="w-full h-full flex justify-center items-center flex-col">
                    "loading..."
                </div>
            }.into_any(),
            Auth::Unauthenticated(_) => view!{
                <div class="w-full h-full flex justify-center items-center gap-2 flex-col">
                    "Unauthorized!"
                    <LoginButton />
                </div>
            }.into_any(),
            Auth::Error(auth_error) => view! {
                <div class="w-full h-full flex justify-center items-center gap-2 flex-col">
                    <h1>"Authorization Error"</h1>
                    <p>{ auth_error.to_string() }</p>
                    <A href="/">"back to the HomePage"</A>
                </div>
            }.into_any(),
        }}
    }
}
