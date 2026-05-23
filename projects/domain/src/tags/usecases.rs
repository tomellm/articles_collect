mod add_new_tag;
mod edit_tag;

pub use add_new_tag::AddTagError;
pub use add_new_tag::add_new_tag;
pub use edit_tag::EditTagError;
pub use edit_tag::edit_tag;

use crate::{
    common::out::StorageError,
    tags::{TagUuid, out::repository::TagsRepository},
};

pub async fn delete_tag(tag_uuid: TagUuid, repo: &impl TagsRepository) -> Result<(), StorageError> {
    repo.delete(tag_uuid).await?;
    tracing::info!("deleted tag with id '{}'", tag_uuid);
    Ok(())
}
