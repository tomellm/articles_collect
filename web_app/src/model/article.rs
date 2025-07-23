use domain::articles::Article;
use leptos::{component, prelude::*, server, view, IntoView};
use leptos_keycloak_auth::components::ShowWhenAuthenticated;
use tracing::info;
use uuid::Uuid;

use crate::utils::{Button, DialogSignal, DialogState};

#[component]
pub fn ArticlesList() -> impl IntoView {
    let articles_fn = OnceResource::new(async { get_articles().await.unwrap() });
    let dialog = expect_context::<DialogSignal>();
    view! {
        <Suspense fallback=|| view! { "loading....." }>
            {Suspend::new(async move {
                let articles = RwSignal::new(articles_fn.await);
                let delete_action = delete_action(articles);
                let open_delete_dialog = open_delete_dialog_action(dialog, delete_action);

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
pub fn ArticleInList(article: Article, open_delete_dialog: Action<Uuid, ()>) -> impl IntoView {
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

pub fn open_delete_dialog_action(
    dialog: DialogSignal,
    delete_action: Action<Uuid, ()>,
) -> Action<Uuid, ()> {
    Action::new(move |uuid: &Uuid| {
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

pub fn delete_action(articles: RwSignal<Vec<Article>>) -> Action<Uuid, ()> {
    Action::new(move |uuid: &Uuid| {
        let uuid = *uuid;
        async move {
            if delete_article(uuid).await.is_ok() {
                articles.update(move |articles| {
                    let _ = articles.extract_if(.., |a| a.uuid.eq(&uuid)).count();
                });
            }
        }
    })
}

#[server]
pub async fn add_articles(file_contents: String) -> Result<(), ServerFnError> {
    use crate::{utils::keycloak_token, ServerState};
    use database::articles_query;

    let ext_token = keycloak_token().await?;
    info!("{ext_token:?}");

    if file_contents.is_empty() {
        return Ok(());
    }

    let state = expect_context::<ServerState>();

    let articles = file_contents
        .lines()
        .map(|line| {
            let line = String::from(line);
            let title = get_title_from_url(line.clone());
            Article::from_parts(title, line)
        })
        .collect();

    Ok(articles_query::insert_many(articles, &state.db).await?)
}

#[server]
pub async fn get_articles() -> Result<Vec<Article>, ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

    let state = expect_context::<ServerState>();
    Ok(articles_query::all(&state.db).await?)
}

#[server]
pub async fn delete_article(article_uuid: Uuid) -> Result<(), ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

    let state = expect_context::<ServerState>();
    Ok(articles_query::delete(article_uuid, &state.db).await?)
}

#[cfg(feature = "ssr")]
fn get_title_from_url(mut url: String) -> String {
    let url = if url.starts_with("https://") {
        let _ = url.drain(0..8);
        url
    } else if url.starts_with("http://") {
        let _ = url.drain(0..7);
        url
    } else {
        url
    };
    let mut parts = url.split('/');
    let first = parts.next().unwrap().to_string();
    let last = parts.rev().find(|p| !p.is_empty() && !p.eq(&first));

    match last {
        Some(last) => format!("{first} - {last}"),
        None => first,
    }
}

#[cfg(test)]
mod tests {
    use crate::model::article::get_title_from_url;

    #[test]
    fn get_title_from_url_https_ending_slash() {
        let url = "https://www.nasa.gov/centers-and-facilities/stennis/stennis-first-open-source-software/".into();
        let res = get_title_from_url(url);
        assert_eq!(
            String::from("www.nasa.gov - stennis-first-open-source-software"),
            res
        );
    }

    #[test]
    fn get_title_from_url_https() {
        let url = "https://github.com/mrkline/modern-latex".into();
        let res = get_title_from_url(url);
        assert_eq!(String::from("github.com - modern-latex"), res);
    }

    #[test]
    fn get_title_from_url_http() {
        let url = "http://github.com/mrkline/modern-latex".into();
        let res = get_title_from_url(url);
        assert_eq!(String::from("github.com - modern-latex"), res);
    }
}
