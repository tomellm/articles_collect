use std::sync::Arc;

use leptos::prelude::*;

use crate::utils::Button;

#[component]
pub fn GlobalDialog(mut dialog: DialogSignal) -> impl IntoView {
    view! {
        <dialog open=move || dialog.is_open()
            class:block=move || dialog.is_open()
            class="bg-black/40 z-200 border-box fixed">
            <div class=" grid place-content-center w-screen h-screen">
                <form method="dialog" class="bg-white">
                    <div class="m-5">
                        <div class="flex justify-between">
                            <h2 class="text-xl">{ move || dialog.title_text() }</h2>
                            <button on:click=move |_| dialog.close()>
                                <Button>
                                    "x"
                                </Button>
                            </button>
                        </div>
                        <h3 class="py-2 text-xl">
                            { move || dialog.content_text() }
                        </h3>
                        <div class="flex justify-between">
                            <button on:click=move |_| dialog.no_action()>
                                <Button>
                                    { move || dialog.no_text() }
                                </Button>
                            </button>
                            <button on:click=move |_| dialog.yes_action()>
                                <Button>
                                    { move || dialog.yes_text() }
                                </Button>
                            </button>
                        </div>
                    </div>
                </form>
            </div>
        </dialog>
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

    pub fn yes_no<YFn, NFn>(yes_action: YFn, no_action: NFn, title: &str, text: &str) -> Self
    where
        YFn: Fn() + Send + Sync + 'static,
        NFn: Fn() + Send + Sync + 'static,
    {
        Self {
            yes_text: String::from("yes"),
            yes_action: Arc::new(yes_action),
            no_text: String::from("no"),
            no_action: Arc::new(no_action),
            text: String::from(text),
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

    pub fn close(&mut self) {
        self.no_action();
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
