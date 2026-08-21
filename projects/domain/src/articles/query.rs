use crate::{
    articles::{Article, ArticleUuid, out::repository::ArticlesRepository},
    common::out::StorageError,
};

pub async fn all(repo: &impl ArticlesRepository) -> Result<Vec<Article>, StorageError> {
    repo.all().await
}

pub async fn one(
    article_uuid: ArticleUuid,
    repo: &impl ArticlesRepository,
) -> Result<Option<Article>, StorageError> {
    repo.one(article_uuid).await
}
