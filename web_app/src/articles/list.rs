use domain::articles::Article;
use leptos::{component, prelude::*, server, view, IntoView};
use leptos_router::components::A;
use uuid::Uuid;

use crate::{
    articles::{delete::list::open_delete_dialog_action_list, ArticleUrl},
    keycloak::ShowWhenAuthenticated,
    utils::{
        dialog::DialogSignal,
        screen_sizes::{use_width, TailwindScreenSizes},
        Button, CenteredLoader,
    },
};

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

#[component]
fn ArticleInList(
    article: RwSignal<Article>,
    open_delete_dialog: Action<Uuid, ()>,
) -> impl IntoView {
    let width = use_width();
    view! {
        <div class="flex flex-col gap-1 p-2 relative overflow-hidden">
            <A href=move || format!("/articles/{}", article.read().uuid)>
                <h3 class="text-2xl text-wrap">{ move || article.get().title }</h3>
            </A>
            <ArticleUrl url=Signal::derive(move || article.get().url) add_classes="md:block hidden" />
            <div class="mx-2 my-1 absolute top-0 right-0 flex gap-2">
                <Show when=width.is_md()>
                    <a href=move || article.get().url target="_blank" class="text-blue-600">
                        <Button>
                            "->"
                        </Button>
                    </a>
                </Show>
                <ShowWhenAuthenticated>
                    <button on:click=move |_| { open_delete_dialog.dispatch(article.read().uuid); }>
                        <Button>
                            "x"
                        </Button>
                    </button>
                </ShowWhenAuthenticated>
            </div>
        </div>
    }
}

#[server(prefix = "/public/api")]
async fn get_articles() -> Result<Vec<Article>, ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

    let state = expect_context::<ServerState>();
    Ok(articles_query::all(&state.db).await?)
}
