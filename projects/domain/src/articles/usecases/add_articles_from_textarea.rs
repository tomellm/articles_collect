use crate::{
    articles::{Article, out::repository::ArticlesRepository},
    common::out::StorageError,
};

pub async fn add_articles_from_textarea(
    text_contents: String,
    repo: &impl ArticlesRepository,
) -> Result<(), StorageError> {
    if text_contents.is_empty() {
        return Ok(());
    }

    let articles = text_contents
        .lines()
        .map(|line| {
            let line = String::from(line);
            let title = get_title_from_url(line.clone());
            Article::from_parts(title, line)
        })
        .collect();

    repo.insert_many(articles).await
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
