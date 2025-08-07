use std::env;

#[cfg(feature = "ssr")]
use axum_keycloak_auth::decode::KeycloakToken;
use futures::{Sink, Stream};
use leptos::{
    prelude::*,
    server_fn::{
        client::{browser::BrowserClient, Client},
        request::browser::BrowserRequest,
        response::browser::BrowserResponse,
    },
    wasm_bindgen::UnwrapThrowExt,
};
use leptos_oidc::{Auth, AuthParameters, AuthSignal, Challenge, LoginLink, LogoutLink};
use leptos_router::components::A;
use serde::{Deserialize, Serialize};
use tracing::error;

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

#[cfg(feature = "ssr")]
pub async fn keycloak_token() -> Result<KeycloakToken<String>, ServerFnError> {
    use axum::Extension;
    use axum_keycloak_auth::decode::KeycloakToken;
    use leptos_axum::extract;
    use tracing::error;

    let Extension(token) = extract::<Extension<KeycloakToken<String>>>()
        .await
        .map_err(|err| {
            error!("error getting keycloak auth token: {err}");
            err
        })?;

    Ok(token)
}

pub struct AuthClient;

impl<E, IS, OS> Client<E, IS, OS> for AuthClient
where
    E: FromServerFnError,
    IS: FromServerFnError,
    OS: FromServerFnError,
{
    type Request = BrowserRequest;
    type Response = BrowserResponse;

    fn send(
        req: Self::Request,
    ) -> impl std::prelude::rust_2024::Future<Output = Result<Self::Response, E>> + Send {
        let headers = req.headers();
        let auth = expect_context::<AuthSignal>()
            .get()
            .authenticated()
            .expect_throw(&format!(
                "You have to be authenticated to make a request to: {}",
                req.url()
            ))
            .id_token();
        headers.append("Authorization", &format!("Bearer {auth}"));
        <BrowserClient as Client<E, IS, OS>>::send(req)
    }

    fn open_websocket(
        path: &str,
    ) -> impl std::prelude::rust_2024::Future<
        Output = Result<
            (
                impl Stream<Item = Result<server_fn::Bytes, server_fn::Bytes>> + Send + 'static,
                impl Sink<server_fn::Bytes> + Send + 'static,
            ),
            E,
        >,
    > + Send {
        <BrowserClient as Client<E, IS, OS>>::open_websocket(path)
    }

    fn spawn(future: impl std::prelude::rust_2024::Future<Output = ()> + Send + 'static) {
        <BrowserClient as Client<E, IS, OS>>::spawn(future)
    }
}
