pub mod out;
pub mod query;
pub mod usecases;

use serde::{Deserialize, Serialize};
use type_state_builder::TypeStateBuilder;

use crate::type_uuid;

type_uuid!(TagUuid);

pub trait ISmallTag: Clone + Send + Sync + 'static {
    fn uuid(&self) -> TagUuid;
    fn name(&self) -> String;
}

impl<T> ISmallTag for T
where
    T: ITag,
{
    fn uuid(&self) -> TagUuid {
        ITag::uuid(self)
    }

    fn name(&self) -> String {
        ITag::name(self)
    }
}

pub trait ITag: Clone + Send + Sync + 'static {
    fn uuid(&self) -> TagUuid;
    fn name(&self) -> String;
    fn description(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, TypeStateBuilder)]
pub struct Tag {
    #[builder(default = TagUuid::new(), impl_into = true)]
    pub uuid: TagUuid,
    #[builder(required)]
    pub name: String,
    #[builder(required)]
    pub description: String,
}

impl ITag for Tag {
    fn uuid(&self) -> TagUuid {
        self.uuid
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, TypeStateBuilder)]
pub struct SimpleTag {
    pub uuid: TagUuid,
    pub name: String,
}

impl ISmallTag for SimpleTag {
    fn uuid(&self) -> TagUuid {
        self.uuid
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TagWithCount {
    pub uuid: TagUuid,
    pub name: String,
    pub description: String,
    /// number of times the tag is used for articles
    pub count: usize,
}

impl ITag for TagWithCount {
    fn uuid(&self) -> TagUuid {
        self.uuid
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
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
