use domain::articles::Article;
use leptos::{form::MultiActionForm, prelude::*, server::ServerMultiAction};
use web_sys::HtmlTextAreaElement;

use crate::{
    keycloak::{AuthClient, ExpectAuth},
    utils::{
        extensions::{MultiactionLastSubSignalExtensions, ServerMultiActionExtensions},
        Button, CenterColumn, CenteredLoader,
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
                links.set(String::new())
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
                        false => AddForm(AddFormProps { links}).into_any(),
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
                        <Button>
                            <button on:click=move |_| {
                                    links.update(|s| s.push('\n'));
                                    textarea_ref.get().map(|t_ref: HtmlTextAreaElement| t_ref.focus());
                                }
                                type="button">
                                "Newline"
                            </button>
                        </Button>
                    </Show>
                    <Button>
                        <input type="submit" value="Send"/>
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

#[server(
    client = AuthClient
)]
async fn add_articles(file_contents: String) -> Result<(), ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

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

#[cfg(any(feature = "ssr", test))]
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
    use super::*;

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
