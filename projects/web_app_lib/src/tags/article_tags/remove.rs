use domain::{
    articles::{Article, ArticleUuid},
    tags::TagUuid,
};
use leptos::prelude::*;

use crate::{
    keycloak::AuthClient,
    utils::dialog::{DialogSignal, DialogState},
};

pub fn open_delete_dialog_action_list(
    dialog: DialogSignal,
    article: RwSignal<Article>,
) -> Action<TagUuid, ()> {
    Action::new(move |uuid: &TagUuid| {
        let remove_tag_fn = move |uuid: TagUuid| {
            article.update(|art| {
                art.tags = art
                    .take_tags()
                    .into_iter()
                    .filter(|t| !t.uuid.eq(&uuid))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
            });
        };
        let uuid = *uuid;
        let delete_action = delete_action_remove_from_list(article.read().uuid, remove_tag_fn);
        async move {
            dialog.open(DialogState::yes(
                move || {
                    delete_action.dispatch(uuid);
                },
                "Remove Tag?",
                "Do you really want to remove this Tag from this Article?",
            ));
        }
    })
}

fn delete_action_remove_from_list(
    article: ArticleUuid,
    remove_tag: impl Fn(TagUuid) + Clone + Send + Sync + 'static,
) -> Action<TagUuid, ()> {
    Action::new(move |uuid: &TagUuid| {
        let uuid = *uuid;
        let remove_tag = remove_tag.clone();
        async move {
            if remove_tag_from_article(article, uuid).await.is_ok() {
                remove_tag(uuid);
            }
        }
    })
}

#[server(
    client = AuthClient
)]
async fn remove_tag_from_article(
    article_uuid: ArticleUuid,
    tag_uuid: TagUuid,
) -> Result<(), ServerFnError> {
    use database::articles_query::ArticlesRepositoryImpl;
    use std::sync::Arc;

    let repo = expect_context::<Arc<ArticlesRepositoryImpl>>();

    domain::articles::usecases::remove_tag(article_uuid, tag_uuid, &*repo).await?;

    Ok(())
}
