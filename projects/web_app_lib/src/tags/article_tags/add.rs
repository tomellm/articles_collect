use std::collections::HashSet;

use crate::{
    keycloak::AuthClient,
    tags::small::TagWithAddition,
    utils::{
        Button, CenteredLoader,
        dialog::{DialogSignal, DialogState},
    },
};
#[cfg(feature = "ssr")]
use domain::tags;
use domain::{
    articles::{Article, ArticleUuid},
    tags::{ITag, Tag, TagUuid},
};
use leptos::prelude::*;

#[component]
pub fn AddTagButton(article: RwSignal<Article>) -> impl IntoView {
    let dialog = expect_context::<DialogSignal>();
    let open_add_action = open_add_dialog_action(dialog, article);
    view! {
        <Button on_click=move || { open_add_action.dispatch(()); }>
            "+"
        </Button>
    }
}

#[component]
pub fn AddTagView(
    tags_to_add: RwSignal<HashSet<TagUuid>>,
    present_tags: Signal<Vec<TagUuid>>,
) -> impl IntoView {
    let tags_fn = OnceResource::new(async move {
        get_tags()
            .await
            .unwrap()
            .into_iter()
            .filter(|t| !present_tags.read().contains(&t.uuid))
            .collect::<Vec<_>>()
    });
    let add_remove_fn = |tag: &TagUuid, set: &mut HashSet<TagUuid>| {
        if set.contains(tag) {
            set.remove(tag);
        } else {
            set.insert(*tag);
        }
    };

    view! {
        <Suspense fallback=CenteredLoader>
            {Suspend::new(async move {
                let tags = RwSignal::new(tags_fn.await);
                match tags.read().is_empty() {
                    true => NoTagView().into_any(),
                    false => view! {
                        <div class="flex gap-6 md:gap-2">
                            <For each=move || tags.get()
                                key=|state| state.uuid
                                let(tag)
                            >
                                <TagWithAddition tag=tag.name.clone()>
                                    <Button on_click=move || { tags_to_add.update(|set| add_remove_fn(&tag.uuid, set)); }>
                                        {move || match tags_to_add.read().contains(&tag.uuid) {
                                            true => "-",
                                            false => "+",
                                        }}
                                    </Button>
                                </TagWithAddition>
                            </For>
                        </div>
                    }.into_any()
                }
             })}
        </Suspense>
    }
}
#[component]
pub fn NoTagView() -> impl IntoView {
    view! {
        <div>
            "There are not tags to add for this Article"
        </div>
    }
}

pub fn open_add_dialog_action(dialog: DialogSignal, article: RwSignal<Article>) -> Action<(), ()> {
    let tags_to_add: RwSignal<HashSet<TagUuid>> = RwSignal::new(HashSet::new());
    let present_tags = Signal::derive(move || {
        article
            .read()
            .tags
            .iter()
            .map(ITag::uuid)
            .collect::<Vec<_>>()
    });
    let add_tags = add_tags_action(article, tags_to_add);

    Action::new(move |_: &()| async move {
        dialog.open(DialogState::yes_content(
            "Add",
            move || {
                tracing::info!("currently {} tags: ", tags_to_add.read().len());
                add_tags.dispatch(());
            },
            "Add a Tag",
            move || {
                view! {
                    <AddTagView tags_to_add present_tags/>
                }
            },
        ));
    })
}

fn add_tags_action(
    article: RwSignal<Article>,
    tags_to_add: RwSignal<HashSet<TagUuid>>,
) -> Action<(), ()> {
    Action::new(move |_: &()| {
        let article_uuid = article.read().uuid;
        let tag_uuids = tags_to_add.get().into_iter().collect::<Vec<_>>();
        async move {
            if tags_to_add.read().is_empty() {
                return;
            }

            if let Ok(mut tags) = add_and_get_tags(article_uuid, tag_uuids).await {
                article.update(|a| {
                    tags.extend(a.take_tags());
                    a.tags = tags.into_boxed_slice();
                });
            }
        }
    })
}

#[server(
    client = AuthClient
)]
async fn get_tags() -> Result<Vec<Tag>, ServerFnError> {
    use database::tags_query::TagsRepositoryImpl;
    use std::sync::Arc;

    let repo = expect_context::<Arc<TagsRepositoryImpl>>();

    tags::query::all(&*repo).await.map_err(ServerFnError::from)
}

/// Adds selected tags to an article and returns the article's current tags.
#[server(client = AuthClient)]
async fn add_and_get_tags(
    article_uuid: ArticleUuid,
    tag_uuids: Vec<TagUuid>,
) -> Result<Vec<Tag>, ServerFnError> {
    use database::{articles_query::ArticlesRepositoryImpl, tags_query::TagsRepositoryImpl};
    use std::sync::Arc;

    let articles_repo = expect_context::<Arc<ArticlesRepositoryImpl>>();
    let tags_repo = expect_context::<Arc<TagsRepositoryImpl>>();

    let tag_uuids = HashSet::from_iter(tag_uuids);

    domain::articles::usecases::add_tags(article_uuid, tag_uuids, &*articles_repo).await?;

    tags::query::all_of_article(&article_uuid, &*tags_repo)
        .await
        .map_err(ServerFnError::from)
}
