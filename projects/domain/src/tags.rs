use serde::{Deserialize, Serialize};
use type_state_builder::TypeStateBuilder;

use crate::type_uuid;

type_uuid!(TagUuid);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, TypeStateBuilder)]
pub struct Tag {
    #[builder(default = TagUuid::new(), impl_into = true)]
    pub uuid: TagUuid,
    #[builder(required)]
    pub name: String,
    #[builder(required)]
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, TypeStateBuilder)]
pub struct SimpleTag {
    pub uuid: TagUuid,
    pub name: String,
}

impl SimpleTag {
    pub fn clone_list(tags: &[Tag]) -> Vec<SimpleTag> {
        tags.iter()
            .map(|tag| {
                SimpleTag::builder()
                    .uuid(tag.uuid)
                    .name(tag.name.clone())
                    .build()
            })
            .collect()
    }
}

impl From<Tag> for SimpleTag {
    fn from(Tag { uuid, name, .. }: Tag) -> Self {
        SimpleTag { uuid, name }
    }
}
