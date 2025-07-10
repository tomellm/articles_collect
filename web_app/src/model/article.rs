use domain::articles::Article;
use leptos::{component, prelude::*, server, view, IntoView};

#[component]
pub fn ArticlesList(articles: Vec<Article>) -> impl IntoView {
    view! {
        <For
            each=move || articles.clone()
            key=|state| state.url.clone()
            let(article)
        >
            <div class="flex gap-2 p-2">
                <h3>{ article.title }</h3>
                <div>{ article.url }</div>
            </div>
        </For>
    }
}

#[component]
pub fn SingleArticle(article: Article) -> impl IntoView {
    view! {
        <p>{ article.url }</p>
    }
}

#[server]
pub async fn add_articles(file_contents: String) -> Result<(), ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

    let state = expect_context::<ServerState>();

    let articles = file_contents
        .lines()
        .map(|line| {
            let line = String::from(line);
            let title = get_title_from_url(line.clone());
            Article::from_into_string(title, line)
        })
        .collect();

    Ok(articles_query::insert_many(articles, &state.db).await?)
}

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

#[server]
pub async fn get_articles() -> Result<Vec<Article>, ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

    let state = expect_context::<ServerState>();
    Ok(articles_query::all(&state.db).await?)
}
