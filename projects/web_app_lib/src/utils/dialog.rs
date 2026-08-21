use std::sync::Arc;

use leptos::prelude::*;

use crate::utils::Button;

/// This is the Global Dialog Html that is displayed when a Dialog is opend, the
/// whole point why the Dialog system Exists. This dialog html is created once
/// and then reused forever
#[component]
pub fn GlobalDialog(dialog: DialogSignal) -> impl IntoView {
    view! {
        // checks if the dialog is open
        <dialog open=move || dialog.is_open()
            class:block=move || dialog.is_open()
            class="bg-black/40 z-200 border-box fixed">
            <div class=" grid place-content-center w-screen h-screen">
                <form method="dialog" class="bg-white">
                    <div class="m-5">
                        <div class="flex justify-between">
                            // shows the title Text
                            <h2 class="text-xl">{ move || dialog.title_text() }</h2>
                            // shows x in the top right corner
                            <Button on_click=move || {
                                let mut dialog = dialog;
                                dialog.close();
                            }>
                                "x"
                            </Button>
                        </div>
                        // The actual content of the dialog
                        <div class="py-2 text-xl">
                            { move || dialog.content() }
                        </div>
                        <div class="flex justify-between">
                            // The "NO" button of the dialog, will only be shown
                            // if there is an actual no text
                            <Show when=move || dialog.no_text().is_some()
                                fallback=EmptyNo>
                                <Button on_click=move || {
                                    let mut dialog = dialog;
                                    dialog.no_action();
                                }>
                                    { move || dialog.no_text() }
                                </Button>
                            </Show>
                            // The "YES" button of the dialog text
                            <Button on_click=move || {
                                let mut dialog = dialog;
                                dialog.yes_action();
                            }>
                                { move || dialog.yes_text() }
                            </Button>
                        </div>
                    </div>
                </form>
            </div>
        </dialog>
    }
}

/// Component that is shown when there is no "NO" text in the dialog. Just
/// a placeholder
#[component]
fn EmptyNo() -> impl IntoView {
    view! { <div class="block"></div> }
}

/// State provided by the user of the context
/// meaning that every time the Dialog is opend this can be different
#[derive(Clone)]
pub struct DialogState {
    /// Text shown in the "YES" button
    yes_text: String,
    /// Action that can be taken with the "YES" button. Does not have to be
    /// set. Consider that, whether this action is set or not, clicking the
    /// button will close the dialog.
    yes_action: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
    /// The text shown in the "NO" button. Can be None, in which case the
    /// whole button is hidden
    no_text: Option<String>,
    /// The "NO" action to be taken when clicking the "NO" button if the text
    /// is set. Consider that, whether this action is set or not, clicking the
    /// button will close the dialog.
    no_action: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
    /// Contains the content that will be shown at the center of the dialog Box
    content: DialogContent,
    /// The title at the top of the dialog box.
    title: String,
}

impl DialogState {
    pub fn yes<YFn>(yes_action: YFn, title: &str, text: &str) -> Self
    where
        YFn: Fn() + Send + Sync + 'static,
    {
        Self {
            yes_text: String::from("yes"),
            yes_action: Some(Arc::new(yes_action)),
            no_text: Some(String::from("no")),
            no_action: Some(Arc::new(|| ())),
            content: DialogContent::from_text(text),
            title: title.into(),
        }
    }

    pub fn yes_content<YFn, C, V>(yes_text: &str, yes_action: YFn, title: &str, content: C) -> Self
    where
        YFn: Fn() + Send + Sync + 'static,
        C: Fn() -> V + Send + Sync + 'static,
        V: IntoView + Send + 'static,
    {
        Self {
            yes_text: yes_text.into(),
            yes_action: Some(Arc::new(yes_action)),
            no_text: None,
            no_action: None,
            content: content.into(),
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
            yes_action: Some(Arc::new(yes_action)),
            no_text: Some(String::from("no")),
            no_action: Some(Arc::new(no_action)),
            content: DialogContent::from_text(text),
            title: title.into(),
        }
    }

    pub fn accept(text: String, title: String) -> Self {
        Self {
            yes_text: String::from("ok"),
            yes_action: None,
            no_text: None,
            no_action: None,
            content: DialogContent::from_text(text),
            title,
        }
    }
}

/// Represents the content shown at the center of the dialog. As long as it
/// can be represented as html in the end it can be in here
#[derive(Clone)]
pub enum DialogContent {
    /// Simple Text content for the dialog, shown in a paragraph
    Text(String),
    /// Any kind of View that would be interesting
    Html(ChildrenFn),
}

impl DialogContent {
    fn from_text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }
}

impl<T, V> From<T> for DialogContent
where
    T: Fn() -> V + Send + Sync + 'static,
    V: IntoView + Send + 'static,
{
    fn from(value: T) -> Self {
        Self::Html(Arc::new(move || value().into_any()))
    }
}

impl Default for DialogContent {
    fn default() -> Self {
        Self::Text(Default::default())
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

    pub fn open(&self, state: DialogState) {
        self.0.set(Some(state));
    }

    pub fn close(&mut self) {
        self.no_action();
    }

    pub fn content(&self) -> AnyView {
        self.0
            .read()
            .as_ref()
            .map(|state| match &state.content {
                DialogContent::Text(text) => view! {
                    <h3 style="white-space: pre;">
                        { text.clone() }
                    </h3>
                }
                .into_any(),
                DialogContent::Html(children) => children().into_any(),
            })
            .unwrap_or("Content Text".into_any())
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
        if let Some(action) = self.0.write().take().and_then(|s| s.yes_action) {
            (action)();
        }
    }

    pub fn no_text(&self) -> Option<String> {
        self.0
            .read()
            .as_ref()
            .and_then(|state| state.no_text.clone())
    }

    pub fn no_action(&mut self) {
        if let Some(action) = self.0.write().take().and_then(|s| s.no_action) {
            (action)();
        }
    }
}

pub trait ErrorDialog {
    fn error(text: &str, err: impl std::fmt::Display) -> Self;
}

impl ErrorDialog for DialogState {
    fn error(text: &str, err: impl std::fmt::Display) -> Self {
        Self::accept(
            format!(
                r#"{text}

{err}"#
            ),
            "Error".into(),
        )
    }
}
