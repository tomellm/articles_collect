use domain::articles::{self};
use leptos::{form::MultiActionForm, prelude::*, server::ServerMultiAction};
use web_sys::HtmlTextAreaElement;

use crate::{
    keycloak::{AuthClient, ExpectAuth},
    utils::{
        Button, CenterColumn, CenteredLoader,
        extensions::{MultiactionLastSubSignalExtensions, ServerMultiActionExtensions},
    },
};

#[component]
pub fn EditArticles() -> impl IntoView {
    let add_articles = ServerMultiAction::<AddArticles>::new();
    let links = RwSignal::new(String::new());

    let submission = add_articles.last_submission_signal();
    let is_pending = submission.pending();
    let state = submission.state();

    Effect::watch(
        state,
        move |val, _, _| {
            if let Some(Ok(())) = val {
                links.set(String::new());
            }
        },
        false,
    );

    view! {
        <CenterColumn>
            <ExpectAuth>
                <MultiActionForm action=add_articles>
                    { move || match is_pending.get() {
                        true => CenteredLoader().into_any(),
                        false => AddForm(AddFormProps { links }).into_any(),
                    }}
                </MultiActionForm>
            </ExpectAuth>
        </CenterColumn>
    }
}

#[component]
fn AddForm(links: RwSignal<String>) -> impl IntoView {
    let textarea_ref = NodeRef::new();

    view! {
        <div class="flex flex-col gap-2 p-2">
            <div class="flex justify-between">
                <label class="underline">
                    "Upload List of Articles"
                </label>
                <div class="flex gap-2">
                    <Show when=move || !links.read().is_empty()>
                        <Button on_click=move || {
                                links.update(|s| s.push('\n'));
                                textarea_ref.get().map(|t_ref: HtmlTextAreaElement| t_ref.focus());
                            }>
                            "Newline"
                        </Button>
                    </Show>
                    <Button button_type="submit">
                        "Send"
                    </Button>
                </div>
            </div>
            <textarea name="file_contents"
                bind:value=links
                node_ref=textarea_ref
                class="field-sizing-content h-100 border-1 border-gray-200">
            </textarea>
        </div>
    }
}

/// Endpoint to add a list of new articles, the file_contents parameter will
/// be parsed as a list of url's that are then loaded a separate articles. The
/// requester needs to be authenticated to make this request
#[server(
    client = AuthClient
)]
async fn add_articles(file_contents: String) -> Result<(), ServerFnError> {
    use database::articles_query::ArticlesRepositoryImpl;
    use std::sync::Arc;

    let repo = expect_context::<Arc<ArticlesRepositoryImpl>>();

    articles::usecases::add_articles_from_textarea(file_contents, &*repo)
        .await
        .map_err(ServerFnError::from)
}
