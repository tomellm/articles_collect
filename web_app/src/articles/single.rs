use domain::articles::Article;
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use uuid::Uuid;

use crate::{articles::ArticleUrl, utils::CenterColumn};

#[derive(Params, PartialEq, Eq)]
pub struct SingleArticleParams {
    uuid: Uuid,
}

#[component]
pub fn SingleArticle() -> impl IntoView {
    let params = use_params::<SingleArticleParams>();

    view! {
        <CenterColumn with_border=false>
            {move || params
                .read()
                .as_ref()
                .ok()
                .map(|params| LoadingArticle(LoadingArticleProps { uuid: params.uuid }).into_any())
                .unwrap_or(
                    view! {
                        <div>
                            "not valid uuid"
                        </div>
                    }
                    .into_any(),
                )
            }
        </CenterColumn>
    }
}

#[component]
pub fn LoadingArticle(uuid: Uuid) -> impl IntoView {
    let article_fn = OnceResource::new(async move { get_article(uuid).await.unwrap() });

    view! {
        <Suspense fallback=|| view! { "loading....." }>
            {Suspend::new(async move { match article_fn.await {
                Some(article) => ArticleView(ArticleViewProps {
                    article: RwSignal::new(article)
                }).into_any(),
                None => NotFound().into_any(),
            }})}
        </Suspense>
    }
}

#[component]
pub fn ArticleView(article: RwSignal<Article>) -> impl IntoView {
    view! {
        <div class="mx-4 mt-6">
            <h1 class="text-5xl wrap-break-word">{move || article.get().title} </h1>
            <ArticleUrl url=Signal::derive(move || article.get().url)
                add_classes="text-3xl wrap-break-word"/>
        </div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div>
            "Not Found ;("
        </div>
    }
}

#[server(prefix = "/public/api")]
async fn get_article(uuid: Uuid) -> Result<Option<Article>, ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

    let state = expect_context::<ServerState>();
    Ok(articles_query::one(&state.db, uuid).await?)
}
