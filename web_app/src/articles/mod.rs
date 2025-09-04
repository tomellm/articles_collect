use leptos::prelude::*;

pub mod delete;
pub mod edit;
pub mod list;
pub mod single;

#[component]
pub fn ArticleUrl(
    url: Signal<String>,
    #[prop(default = "")] add_classes: &'static str,
) -> impl IntoView {
    view! {
        <a href=url target="_blank"
            class=move || ["text-xl text-blue-600", add_classes].join(" ")
        >
            { move || format!("[{}]", url.read()) }
        </a>
    }
}
