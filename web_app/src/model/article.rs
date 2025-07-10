use domain::articles::Article;
use leptos::{component, prelude::*, server, view, IntoView};

#[component]
pub fn ArticlesList(articles: Vec<Article>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <For each=move || articles.clone()
                key=|state| state.url.clone()
                let(article)
            >
                <div class="flex flex-col gap-1 p-2">
                    <h3 class="text-xl">{ article.title }</h3>
                    <a href=article.url target="_blank"
                        class="text-blue-600"
                    >
                        { format!("[{}]", article.url) }
                    </a>
                </div>
            </For>
        </div>
    }
}

#[server]
pub async fn add_articles(file_contents: String) -> Result<(), ServerFnError> {
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

#[server]
pub async fn get_articles() -> Result<Vec<Article>, ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

    let state = expect_context::<ServerState>();
    Ok(articles_query::all(&state.db).await?)
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
