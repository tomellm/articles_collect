use domain::articles::Article;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params, params::Params};
use uuid::Uuid;

use crate::{
    articles::{delete::single::open_delete_dialog_action, ArticleUrl},
    keycloak::ShowWhenAuthenticated,
    utils::{busy_container::BusyContainer, Button, CenterColumn},
};

#[derive(Params, PartialEq, Eq)]
pub struct SingleArticleParams {
    uuid: Uuid,
}

#[component]
pub fn SingleArticle() -> impl IntoView {
    let params = use_params::<SingleArticleParams>();

    view! {
        <CenterColumn with_border=false>
            <div class="mx-4 mt-6">
                {move || params
                    .read()
                        .as_ref()
                        .ok()
                        .map(|params| LoadingArticle(LoadingArticleProps { uuid: params.uuid }).into_any())
                        .unwrap_or( view! { <Title> "Not a valid Uuid" </Title> }.into_any())
                }
            </div>
        </CenterColumn>
    }
}

#[component]
pub fn LoadingArticle(uuid: Uuid) -> impl IntoView {
    let article_fn = OnceResource::new(async move { get_article(uuid).await.unwrap() });

    view! {
        <Suspense fallback=|| view! { <Title>"Loading....."</Title> }>
            {Suspend::new(async move { match article_fn.await {
                Some(article) => ArticleView(ArticleViewProps {
                    article: RwSignal::new(article)
                }).into_any(),
                None => NotFound(NotFoundProps { uuid }).into_any(),
            }})}
        </Suspense>
    }
}

#[component]
pub fn ArticleView(article: RwSignal<Article>) -> impl IntoView {
    let delete_dialog = open_delete_dialog_action(None);

    view! {
        <BusyContainer busy_state=delete_dialog>
            <div class="flex">
                <A href="/articles">
                    <Button>
                    "<-"
                    </Button>
                </A>
            </div>
            <div class="mb-4">
                <Title> {move || article.get().title} </Title>
                <ArticleUrl url=Signal::derive(move || article.get().url)
                    add_classes="text-3xl wrap-break-word" />
            </div>
            <div class="flex flex-row-reverse gap-2">
                <ShowWhenAuthenticated>
                    <button on:click=move |_| {
                        delete_dialog.open_dialog(article.read().uuid);
                    }>
                        <Button>
                            "delete"
                        </Button>
                    </button>
                </ShowWhenAuthenticated>
                <a href=move || article.get().url target="_blank">
                    <Button>
                        <div>"open"</div>
                    </Button>
                </a>
            </div>
        </BusyContainer>
    }
}

#[component]
fn Title(children: ChildrenFn) -> impl IntoView {
    view! {
        <h1 class="text-5xl wrap-break-word">
            {children()}
        </h1>
    }
}

#[component]
pub fn NotFound(uuid: Uuid) -> impl IntoView {
    view! {
        <Title>"Not Found ;("</Title>
        <p class="text-xl">{format!("Article with the uuid: {uuid} could not be found...")}</p>
    }
}

#[server(prefix = "/public/api")]
async fn get_article(uuid: Uuid) -> Result<Option<Article>, ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

    let state = expect_context::<ServerState>();
    Ok(articles_query::one(&state.db, uuid).await?)
}
