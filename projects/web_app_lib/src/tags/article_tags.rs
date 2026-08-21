mod add;
mod remove;

use domain::{articles::Article, tags::SimpleTag};
use leptos::prelude::*;

use crate::{
    keycloak::ShowWhenAuthenticated,
    tags::{
        article_tags::{add::AddTagButton, remove::open_delete_dialog_action_list},
        small::{SmallTag, TagWithAddition},
    },
    utils::dialog::DialogSignal,
};

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
pub fn SmallTagsListWithControls(article: RwSignal<Article>) -> impl IntoView {
    let dialog = expect_context::<DialogSignal>();
    let tags = Signal::derive(move || SimpleTag::clone_list(&article.read().tags));
    let open_delete_dialog = open_delete_dialog_action_list(dialog, article);
    view! {
        <div class="flex gap-2">
            <For each=move || tags.get()
                key=|state| state.uuid
                let(tag)>
                <TagWithAddition tag=tag.name.clone()>
                    <ShowWhenAuthenticated>
                        <button on:click=move |_| { open_delete_dialog.dispatch(tag.uuid); }>
                            "x"
                        </button>
                    </ShowWhenAuthenticated>
                </TagWithAddition>
            </For>
            <ShowWhenAuthenticated>
                <AddTagButton article />
            </ShowWhenAuthenticated>
        </div>
    }
}
