use crate::{
    articles::{ArticleUuid, out::repository::ArticlesRepository},
    common::out::StorageError,
    tags::TagUuid,
};

pub async fn remove_tag(
    article_uuid: ArticleUuid,
    tag_uuid: TagUuid,
    repo: &impl ArticlesRepository,
) -> Result<(), RemoveTagError> {
    if !repo
        .has_tag(article_uuid, tag_uuid)
        .await
        .map_err(RemoveTagError::from)?
    {
        return Err(RemoveTagError::TagNotAssigned(tag_uuid));
    }

    repo.remove_tag(article_uuid, tag_uuid)
        .await
        .map_err(RemoveTagError::from)?;

    tracing::info!(
        "removed tag with id '{}' from article with id '{}'",
        tag_uuid,
        article_uuid
    );

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveTagError {
    #[error("the Tag with uuid {0} is not assigned to this article")]
    TagNotAssigned(TagUuid),
    #[error("{0}")]
    Db(StorageError),
}

impl From<StorageError> for RemoveTagError {
    fn from(value: StorageError) -> Self {
        Self::Db(value)
    }
}
