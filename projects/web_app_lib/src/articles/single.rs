#[cfg(feature = "ssr")]
use domain::articles;
use domain::articles::{Article, ArticleUuid};
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params, params::Params};

use crate::{
    articles::{ArticleUrl, delete::single::open_delete_dialog_action},
    keycloak::ShowWhenAuthenticated,
    tags::article_tags::SmallTagsListWithControls,
    utils::{Button, CenterColumn, NoButton, busy_container::BusyContainer},
};

#[derive(Params, PartialEq, Eq)]
pub struct SingleArticleParams {
    uuid: ArticleUuid,
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
pub fn LoadingArticle(uuid: ArticleUuid) -> impl IntoView {
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
                    <NoButton>
                        "<-"
                    </NoButton>
                </A>
            </div>
            <div class="mb-4">
                <Title> {move || article.get().title} </Title>
                <ArticleUrl url=Signal::derive(move || article.get().url)
                    add_classes="text-3xl wrap-break-word" />
                <SmallTagsListWithControls article/>
            </div>
            <div class="flex flex-row-reverse gap-2">
                <ShowWhenAuthenticated>
                    <Button on_click=move || {
                        delete_dialog.open_dialog(article.read().uuid);
                    }>
                        "delete"
                    </Button>
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
pub fn NotFound(uuid: ArticleUuid) -> impl IntoView {
    view! {
        <Title>"Not Found ;("</Title>
        <p class="text-xl">{format!("Article with the uuid: {uuid} could not be found...")}</p>
    }
}

#[server(prefix = "/public/api")]
async fn get_article(uuid: ArticleUuid) -> Result<Option<Article>, ServerFnError> {
    use database::articles_query::ArticlesRepositoryImpl;
    use std::sync::Arc;

    let repo = expect_context::<Arc<ArticlesRepositoryImpl>>();

    articles::query::one(uuid, &*repo)
        .await
        .map_err(ServerFnError::from)
}
