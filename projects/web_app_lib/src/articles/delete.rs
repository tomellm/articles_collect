pub mod list;
pub mod single;

use domain::articles::{self, ArticleUuid};
use leptos::{prelude::*, server};

use crate::keycloak::AuthClient;

/// Endpoint for deleting articles. The requester needs to be authenticated to
/// make this request
#[server(
    client = AuthClient
)]
async fn delete_article(article_uuid: ArticleUuid) -> Result<(), ServerFnError> {
    use database::articles_query::ArticlesRepositoryImpl;
    use std::sync::Arc;

    let repo = expect_context::<Arc<ArticlesRepositoryImpl>>();

    articles::usecases::delete(article_uuid, &*repo)
        .await
        .map_err(ServerFnError::from)
}
