use domain::articles::Article;
use leptos::{prelude::*, server};
use uuid::Uuid;

use crate::{
    keycloak::AuthClient,
    utils::{DialogSignal, DialogState},
};

pub fn open_delete_dialog_action(
    dialog: DialogSignal,
    articles: RwSignal<Vec<RwSignal<Article>>>,
) -> Action<Uuid, ()> {
    let delete_action = delete_action(articles);

    Action::new(move |uuid: &Uuid| {
        let uuid = *uuid;
        async move {
            dialog.open(DialogState::yes(
                move || {
                    delete_action.dispatch(uuid);
                },
                "Delete Item?",
                "Do you really want to delete this Item?",
            ));
        }
    })
}

fn delete_action(articles: RwSignal<Vec<RwSignal<Article>>>) -> Action<Uuid, ()> {
    Action::new(move |uuid: &Uuid| {
        let uuid = *uuid;
        async move {
            if delete_article(uuid).await.is_ok() {
                articles.update(move |articles| {
                    let _ = articles.extract_if(.., |a| a.read().uuid.eq(&uuid)).count();
                });
            }
        }
    })
}

#[server(
    client = AuthClient
)]
async fn delete_article(article_uuid: Uuid) -> Result<(), ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

    let state = expect_context::<ServerState>();
    Ok(articles_query::delete(article_uuid, &state.db).await?)
}
