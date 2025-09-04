use domain::articles::Article;
use leptos::prelude::*;
use uuid::Uuid;

use crate::{
    articles::delete::delete_article,
    utils::dialog::{DialogSignal, DialogState},
};

pub fn open_delete_dialog_action_list(
    dialog: DialogSignal,
    articles: RwSignal<Vec<RwSignal<Article>>>,
) -> Action<Uuid, ()> {
    let delete_action = delete_action_remove_from_list(articles);

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

fn delete_action_remove_from_list(articles: RwSignal<Vec<RwSignal<Article>>>) -> Action<Uuid, ()> {
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
