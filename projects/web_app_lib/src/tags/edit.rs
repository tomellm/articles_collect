mod add;
mod edit;
mod form;

use domain::tags::{self, TagUuid, TagWithCount};
use leptos::prelude::*;

use crate::{
    keycloak::{AuthClient, ExpectAuth},
    tags::{
        delete::open_delete_dialog_action_list,
        edit::{add::ShowAddForm, edit::ShowEditForm},
    },
    utils::{CenterColumn, CenteredLoader, dialog::DialogSignal},
};

/// an enumeration of the different states that the tags editing page can be in
#[derive(Default)]
enum CurrentState {
    /// the tag that is currently beeing edited still needs to be added to
    /// the database
    #[default]
    Add,
    /// the tag that is currently beeing edited is already presentin the
    /// database, it just needs an update
    Edit,
}

impl CurrentState {
    /// Returns if this [`CurrentState`] is in the Add mode
    pub fn is_add(&self) -> bool {
        matches!(self, Self::Add)
    }

    /// Returns if this [`CurrentState`] is in the Edit mode
    pub fn is_edit(&self) -> bool {
        matches!(self, Self::Edit)
    }
}

/// The root level component that will show all the tags as well as a view
/// to edit and add new tags.
///
/// # Panics
///
/// Panics if loading the tags fails
#[component]
pub fn EditTags() -> impl IntoView {
    let dialog = expect_context::<DialogSignal>();
    let tag_changes: RwSignal<(usize, Option<bool>)> = RwSignal::new((0usize, None));
    let current_state = RwSignal::new(CurrentState::default());
    let new_result: Action<bool, ()> = Action::new(move |result: &bool| {
        let result = *result;
        async move {
            tag_changes.update(|(version, prev_result)| {
                *version += 1;
                let _ = prev_result.insert(result);
            });
        }
    });

    let tags_fn = Resource::new_blocking(
        move || tag_changes.get(),
        |_| async { get_tags_with_count().await.unwrap() },
    );

    let tag_uuid: RwSignal<Option<TagUuid>> = RwSignal::new(None);
    let tag_name = RwSignal::new(String::new());
    let tag_description = RwSignal::new(String::new());

    let edit_tag = Action::new(move |tag: &TagWithCount| {
        let uuid = tag.uuid;
        let name = tag.name.clone();
        let description = tag.description.clone();
        async move {
            current_state.set(CurrentState::Edit);
            tag_uuid.set(Some(uuid));
            tag_name.set(name);
            tag_description.set(description);
        }
    });

    Effect::watch(
        tag_changes,
        move |val, _, _| match val {
            (_, Some(true)) => {
                current_state.set(CurrentState::Add);
                tag_uuid.set(None);
                tag_name.set(String::new());
                tag_description.set(String::new());
            }
            (_, Some(false)) => (),
            (_, None) => (),
        },
        false,
    );
    view! {
        <CenterColumn>
            <ExpectAuth>
                <div class="p-2">
                    <Suspense fallback=CenteredLoader>
                        {move || { tags_fn.get().map(|tags| {
                            view! { <EditTagsList tags edit_tag dialog/> }
                        }) }}
                    </Suspense>
                    <div class="pb-15 mx-15 mb-15 border-b-2 block"></div>
                    <Show when=move || current_state.read().is_add()>
                        <ShowAddForm uuid=tag_uuid
                            name=tag_name
                            description=tag_description
                            new_result />
                    </Show>
                    <Show when=move || current_state.read().is_edit()>
                        <ShowEditForm uuid=tag_uuid
                            name=tag_name
                            description=tag_description
                            new_result />
                    </Show>

                </div>
            </ExpectAuth>
        </CenterColumn>
    }
}

/// The reactive list of tags. The tags can be deleted or given up for editing
/// which will then lead to a reload of the tags
#[component]
fn EditTagsList(
    tags: Vec<TagWithCount>,
    edit_tag: Action<TagWithCount, ()>,
    dialog: DialogSignal,
) -> impl IntoView {
    let tags = RwSignal::new(tags.into_iter().map(RwSignal::new).collect::<Vec<_>>());
    let open_delete_dialog = open_delete_dialog_action_list(dialog, tags);
    view! {
        <Show
            when=move || !tags.read().is_empty()
            fallback=NoTags >
            <div class="flex flex-wrap gap-6 md:gap-2">
                <For each=move || tags.get()
                    key=|state| state.read().uuid
                    let(tag)
                >
                    <TagWithCount tag edit_tag open_delete_dialog/>
                </For>
            </div>
        </Show>
    }
}

/// The display component of single tag, shows the tag, controls as well as
/// the count of how many articles actually use this tag.
#[component]
fn TagWithCount(
    tag: RwSignal<TagWithCount>,
    edit_tag: Action<TagWithCount, ()>,
    open_delete_dialog: Action<TagUuid, ()>,
) -> impl IntoView {
    view! {
        <div class="flex justify-content items-center py-0.5 px-1 gap-2">
            <div class="border-2 py-1 px-2 flex gap-2">
                { move || tag.get().name }
                <button on:click=move |_| { edit_tag.dispatch(tag.get()); }>
                    "✎"
                </button>
                <button on:click=move |_| { open_delete_dialog.dispatch(tag.read().uuid); }>
                    "x"
                </button>
            </div>
            <div>
                { move || tag.read().count }
            </div>
        </div>
    }
}

/// Fallback component if no tags have been found
#[component]
fn NoTags() -> impl IntoView {
    view! {
        <div class="text-center">
            "you have no tags..."
        </div>
    }
}

/// Fetches all of the tags in the database with the count of articles that
/// each use the tags. The authentication is not nessesary but since this
/// endpoint is not used anywhere else it also doesnt bother
#[server(
    client = AuthClient
)]
async fn get_tags_with_count() -> Result<Vec<TagWithCount>, ServerFnError> {
    use database::tags_query::TagsRepositoryImpl;
    use std::sync::Arc;

    let repo = expect_context::<Arc<TagsRepositoryImpl>>();

    tags::query::all_with_count(&*repo)
        .await
        .map_err(ServerFnError::from)
}
