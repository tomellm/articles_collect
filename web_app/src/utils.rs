use std::sync::Arc;

#[cfg(feature = "ssr")]
use axum_keycloak_auth::decode::KeycloakToken;
use leptos::prelude::*;
use tracing::info;

#[component]
pub fn CenterColumn(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <div class="xl:max-w-[1000px] w-full bg-white border-x-2">
                <div class="pt-25">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Button(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="flex justify-content items-center border-2 py-1 px-2
            hover:bg-black hover:text-white bg-white
            hover:border-l-gray-200 hover:border-b-gray-200
            hover:border-t-gray-950 hover:border-r-gray-950
            ">
            { children() }
        </div>
    }
}

/// State provided by the user of the context
/// meaning that every time the Dialog is opend this can be different
#[derive(Clone)]
pub struct DialogState {
    yes_text: String,
    yes_action: Arc<dyn Fn() + Send + Sync + 'static>,
    no_text: String,
    no_action: Arc<dyn Fn() + Send + Sync + 'static>,
    text: String,
    title: String,
}

impl DialogState {
    pub fn yes<YFn>(yes_action: YFn, title: &str, text: &str) -> Self
    where
        YFn: Fn() + Send + Sync + 'static,
    {
        Self {
            yes_text: String::from("yes"),
            yes_action: Arc::new(yes_action),
            no_text: String::from("no"),
            no_action: Arc::new(|| ()),
            text: text.into(),
            title: title.into(),
        }
    }

    pub fn yes_no<YFn, NFn>(yes_action: YFn, no_action: NFn, title: &str, text: String) -> Self
    where
        YFn: Fn() + Send + Sync + 'static,
        NFn: Fn() + Send + Sync + 'static,
    {
        Self {
            yes_text: String::from("yes"),
            yes_action: Arc::new(yes_action),
            no_text: String::from("no"),
            no_action: Arc::new(no_action),
            text,
            title: title.into(),
        }
    }

    pub fn debug() -> Self {
        Self {
            yes_text: String::from("yes"),
            yes_action: Arc::new(|| ()),
            no_text: String::from("no"),
            no_action: Arc::new(|| ()),
            text: String::from("text"),
            title: String::from("title"),
        }
    }
}

/// Dialog context that contains all of the persistent state about the
/// Dialog, meaning stuff that doesnt change between uses of the Dialog
#[derive(Copy, Clone, Default)]
pub struct DialogSignal(pub RwSignal<Option<DialogState>>);

impl DialogSignal {
    pub fn is_open(&self) -> bool {
        self.0.read().is_some()
    }

    pub fn is_closed(&self) -> bool {
        self.0.read().is_none()
    }

    pub fn open(&self, state: DialogState) {
        self.0.set(Some(state));
    }

    pub fn open_debug(&self) {
        self.0.set(Some(DialogState::debug()));
    }

    pub fn close(&self) {
        info!("closing dialog");
        self.0.set(None);
    }

    pub fn content_text(&self) -> String {
        self.0
            .read()
            .as_ref()
            .map(|state| state.text.clone())
            .unwrap_or("Content Text".into())
    }

    pub fn title_text(&self) -> String {
        self.0
            .read()
            .as_ref()
            .map(|state| state.title.clone())
            .unwrap_or("Title Text".into())
    }

    pub fn yes_text(&self) -> String {
        self.0
            .read()
            .as_ref()
            .map(|state| state.yes_text.clone())
            .unwrap_or("Yes".into())
    }

    pub fn yes_action(&mut self) {
        if let Some(state) = self.0.write().take() {
            (state.yes_action)();
        }
    }

    pub fn no_text(&self) -> String {
        self.0
            .read()
            .as_ref()
            .map(|state| state.no_text.clone())
            .unwrap_or("No".into())
    }

    pub fn no_action(&mut self) {
        if let Some(state) = self.0.write().take() {
            (state.no_action)();
        }
    }
}

#[cfg(feature = "ssr")]
pub async fn keycloak_token() -> Result<KeycloakToken<String>, ServerFnError> {
    use axum::Extension;
    use axum_keycloak_auth::decode::KeycloakToken;
    use leptos_axum::extract;

    Ok(extract::<Extension<KeycloakToken<String>>>().await?.0)
}
