use domain::articles::Article;
use leptos::{component, prelude::*, server, view, IntoView};
use uuid::Uuid;

use crate::{
    articles::delete::open_delete_dialog_action,
    keycloak::ShowWhenAuthenticated,
    utils::{Button, DialogSignal},
};

#[component]
pub fn ArticlesList() -> impl IntoView {
    let articles_fn = OnceResource::new(async { get_articles().await.unwrap() });
    let dialog = expect_context::<DialogSignal>();
    view! {
        <Suspense fallback=|| view! { "loading....." }>
            {Suspend::new(async move {
                let articles = RwSignal::new(articles_fn.await);
                let open_delete_dialog = open_delete_dialog_action(dialog, articles);

                { view! {
                    <div class="flex flex-col gap-2">
                        <For each=move || articles.get()
                            key=|state| state.uuid
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
fn ArticleInList(article: Article, open_delete_dialog: Action<Uuid, ()>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1 p-2 relative overflow-hidden">
            <h3 class="text-xl text-wrap">{ article.title }</h3>
            <a href=article.url target="_blank"
                class="text-blue-600 xl:block hidden"
            >
                { format!("[{}]", article.url) }
            </a>
            <ShowWhenAuthenticated>
                <div class="mx-2 my-1 absolute top-0 right-0">
                    <button on:click=move |_| { open_delete_dialog.dispatch(article.uuid); }>
                        <Button>
                            "x"
                        </Button>
                    </button>
                </div>
            </ShowWhenAuthenticated>
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
