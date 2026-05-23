use domain::articles::{self, Article, ArticleUuid};
use leptos::{IntoView, component, prelude::*, server, view};
use leptos_router::components::A;

use crate::{
    articles::{ArticleUrl, delete::list::open_delete_dialog_action_list},
    keycloak::ShowWhenAuthenticated,
    tags::article_tags::SmallTagsList,
    utils::{
        Button, CenteredLoader, NoButton,
        dialog::DialogSignal,
        screen_sizes::{TailwindScreenSizes, use_width},
    },
};

/// Component that loads the articles from the backend and then displays them
/// in a list of [ArticleInList] components
///
/// # Panics
///
/// Panics if fetching the list of articles returns an error, since that is
/// expected to work.
#[component]
pub fn ArticlesList() -> impl IntoView {
    let articles_fn = OnceResource::new(async { get_articles().await.unwrap() });
    let dialog = expect_context::<DialogSignal>();
    view! {
        <Suspense fallback=CenteredLoader>
            {Suspend::new(async move {
                let articles = articles_fn.await
                    .into_iter()
                    .map(RwSignal::new)
                    .collect::<Vec<_>>();
                let articles = RwSignal::new(articles);
                let open_delete_dialog = open_delete_dialog_action_list(dialog, articles);

                { view! {
                    <div class="flex flex-col gap-6 md:gap-2">
                        <For each=move || articles.get()
                            key=|state| state.read().uuid
                            let(article)
                        >
                            <ArticleInList article open_delete_dialog/>
                        </For>
                    </div>
                }}
             })}
        </Suspense>
    }
}

/// The component to display an article in a list of articles. The component
/// will adapt to the width of the screen, meaning that for wider screens the
/// url of the article will be shown directly, otherwise an arrow will appear
/// to allow the use to navigate to the article directly.
///
/// The component also contains functionality to navigate to the articles page
/// on this website, als well to delete the article if authenticated.
#[component]
fn ArticleInList(
    article: RwSignal<Article>,
    open_delete_dialog: Action<ArticleUuid, ()>,
) -> impl IntoView {
    let width = use_width();
    view! {
        <div class="flex flex-col gap-1 p-2 relative overflow-hidden">
            <A href=move || format!("/articles/{}", article.read().uuid)>
                <h3 class="text-2xl text-wrap">{ move || article.get().title }</h3>
            </A>
            <SmallTagsList article/>
            <ArticleUrl url=Signal::derive(move || article.get().url) add_classes="md:block hidden" />
            <div class="mx-2 my-1 absolute top-0 right-0 flex gap-2">
                <Show when=width.is_md()>
                    <a href=move || article.get().url target="_blank" class="text-blue-600">
                        <NoButton>
                            "->"
                        </NoButton>
                    </a>
                </Show>
                <ShowWhenAuthenticated>
                    <Button on_click=move || { open_delete_dialog.dispatch(article.read().uuid); }>
                        "x"
                    </Button>
                </ShowWhenAuthenticated>
            </div>
        </div>
    }
}

/// Endpoint to request the current list of articles. Will return the complete
/// list of articles. The requester does not need to be authenticated
#[server(prefix = "/public/api")]
async fn get_articles() -> Result<Vec<Article>, ServerFnError> {
    use database::articles_query::ArticlesRepositoryImpl;
    use std::sync::Arc;

    let repo = expect_context::<Arc<ArticlesRepositoryImpl>>();

    articles::query::all(&*repo)
        .await
        .map_err(ServerFnError::from)
}
