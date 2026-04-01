pub mod list;
pub mod single;

use domain::articles::ArticleUuid;
use leptos::{prelude::*, server};

use crate::keycloak::AuthClient;

#[server(
    client = AuthClient
)]
async fn delete_article(article_uuid: ArticleUuid) -> Result<(), ServerFnError> {
    use crate::ServerState;
    use database::articles_query;

    let state = expect_context::<ServerState>();
    Ok(articles_query::delete(article_uuid, &state.db).await?)
}

#[cfg(test)]
mod tests {

    // https://github.com/SeaQL/sea-orm/pull/2590
    //
    // waiting for this pr to go through before writing tests
    //
    //fn delete_article_server_func_deletes_article() {
    //    let mock_db =
    //        MockDatabase::new(DatabaseBackend::Postgres).append_exec_results([MockExecResult {
    //            last_insert_id: 0,
    //            rows_affected: 1,
    //        }]);
    //}
}
