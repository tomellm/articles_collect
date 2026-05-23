use crate::{
    articles::{ArticleUuid, out::repository::ArticlesRepository},
    common::out::StorageError,
};
pub async fn delete(
    article_uuid: ArticleUuid,
    repo: &impl ArticlesRepository,
) -> Result<(), StorageError> {
    repo.delete(article_uuid).await?;
    tracing::info!("deleted article with id '{}'", article_uuid);
    Ok(())
}
