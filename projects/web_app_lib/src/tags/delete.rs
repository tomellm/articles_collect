#[cfg(feature = "ssr")]
use domain::tags;
use domain::tags::{TagUuid, TagWithCount};
use leptos::prelude::*;

use crate::{
    keycloak::AuthClient,
    utils::dialog::{DialogSignal, DialogState},
};

#[server(
    client = AuthClient
)]
async fn delete_tag(tag_uuid: TagUuid) -> Result<(), ServerFnError> {
    use database::tags_query::TagsRepositoryImpl;
    use std::sync::Arc;

    let repo = expect_context::<Arc<TagsRepositoryImpl>>();

    tags::usecases::delete_tag(tag_uuid, &*repo).await?;

    Ok(())
}

pub fn open_delete_dialog_action_list(
    dialog: DialogSignal,
    tags: RwSignal<Vec<RwSignal<TagWithCount>>>,
) -> Action<TagUuid, ()> {
    let delete_action = delete_action_remove_from_list(tags);

    Action::new(move |uuid: &TagUuid| {
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

fn delete_action_remove_from_list(
    tags: RwSignal<Vec<RwSignal<TagWithCount>>>,
) -> Action<TagUuid, ()> {
    Action::new(move |uuid: &TagUuid| {
        let uuid = *uuid;
        async move {
            if delete_tag(uuid).await.is_ok() {
                tags.update(move |tags| {
                    let _ = tags.extract_if(.., |a| a.read().uuid.eq(&uuid)).count();
                });
            }
        }
    })
}
