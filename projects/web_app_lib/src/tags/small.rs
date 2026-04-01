use domain::{articles::Article, tags::SimpleTag};
use leptos::prelude::*;

#[component]
pub fn SmallTagsList(article: RwSignal<Article>) -> impl IntoView {
    let tags = Signal::derive(move || SimpleTag::clone_list(&article.read().tags));
    view! {
        <div>
            <For each=move || tags.get()
                key=|state| state.uuid
                let(tag)>
                <SmallTag tag />
            </For>
        </div>
    }
}

#[component]
pub fn SmallTag(tag: SimpleTag) -> impl IntoView {
    view! {
        <div>{tag.name}</div>
    }
}
