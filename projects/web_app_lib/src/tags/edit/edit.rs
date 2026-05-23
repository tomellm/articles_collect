#[cfg(feature = "ssr")]
use domain::tags;
use domain::tags::TagUuid;
use leptos::prelude::*;

use crate::{
    keycloak::AuthClient,
    tags::edit::form::EditOrAddTagForm,
    utils::{
        CenteredLoader,
        dialog::{DialogSignal, DialogState, ErrorDialog},
        extensions::{MultiactionLastSubSignalExtensions, ServerMultiActionExtensions},
    },
};

#[component]
pub fn ShowEditForm(
    uuid: RwSignal<Option<TagUuid>>,
    name: RwSignal<String>,
    description: RwSignal<String>,
    new_result: Action<bool, ()>,
) -> impl IntoView {
    let dialog = expect_context::<DialogSignal>();

    let edit_tag = ServerMultiAction::<EditTag>::new();

    let submission = edit_tag.last_submission_signal();
    let is_pending = submission.pending();
    let state = submission.state();

    Effect::watch(
        state,
        move |val, _, _| match val {
            Some(Ok(())) => {
                new_result.dispatch(true);
            }
            Some(Err(err)) => {
                dialog.open(DialogState::error("tag could not be edited", err));
                new_result.dispatch(false);
            }
            None => (),
        },
        false,
    );

    view! {
        <MultiActionForm action=edit_tag>
            { move || match is_pending.get() {
                true => CenteredLoader().into_any(),
                false => view! {
                    <EditOrAddTagForm uuid name description new_result
                        submit_text="Update Tag"/>
                }.into_any(),
            }}
        </MultiActionForm>
    }
}

#[server(
    client = AuthClient
)]
async fn edit_tag(
    tag_uuid: String,
    tag_name: String,
    tag_description: String,
) -> Result<(), ServerFnError> {
    use database::tags_query::TagsRepositoryImpl;
    use std::sync::Arc;

    let repo = expect_context::<Arc<TagsRepositoryImpl>>();

    tags::usecases::edit_tag(tag_uuid, tag_name, tag_description, &*repo).await?;

    Ok(())
}
