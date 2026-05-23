use crate::{
    articles::ArticleUuid,
    common::out::StorageError,
    tags::{Tag, TagWithCount, out::repository::TagsRepository},
};

pub async fn all(repo: &impl TagsRepository) -> Result<Vec<Tag>, StorageError> {
    repo.all().await
}

pub async fn all_with_count(repo: &impl TagsRepository) -> Result<Vec<TagWithCount>, StorageError> {
    repo.all_with_count().await
}

/// Returns every tag currently assigned to one article.
pub async fn all_of_article(
    article_uuid: &ArticleUuid,
    repo: &impl TagsRepository,
) -> Result<Vec<Tag>, StorageError> {
    repo.all_of_article(article_uuid).await
}
