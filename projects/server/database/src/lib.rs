pub mod articles_query;
mod entities;
pub mod tags_query;

use std::sync::Arc;

use domain::{articles::ArticleUuid, tags::TagUuid};
use sea_orm::DatabaseConnection;

use crate::{articles_query::ArticlesRepositoryImpl, tags_query::TagsRepositoryImpl};

/// struct containing all different database repositories as arcs
#[derive(Clone)]
pub struct DbState {
    /// articles repository
    pub articles: Arc<ArticlesRepositoryImpl>,
    /// tags repository
    pub tags: Arc<TagsRepositoryImpl>,
}

impl From<DatabaseConnection> for DbState {
    fn from(value: DatabaseConnection) -> Self {
        let tags = Arc::new(TagsRepositoryImpl::new(value.clone()));
        Self {
            tags: Arc::clone(&tags),
            articles: Arc::new(ArticlesRepositoryImpl::new(value, tags)),
        }
    }
}

uuid_db_impls!(ArticleUuid);
uuid_db_impls!(TagUuid);

#[macro_export]
macro_rules! uuid_db_impls {
    ($type:ident) => {
        ::paste::paste! {
            #[repr(transparent)]
            #[derive(
                Debug,
                Clone,
                Copy,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
                ::sea_orm::DeriveValueType,
            )]
            //#[sea_orm(value_type = "String")]
            pub struct [<Db $type>](::uuid::Uuid);

            impl ::std::ops::Deref for [<Db $type>] {
                type Target = ::uuid::Uuid;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl From<$type> for [<Db $type>] {
                fn from(value: $type) -> Self {
                    Self(std::convert::Into::<::uuid::Uuid>::into(*value))
                }
            }

            impl From<[<Db $type>]> for $type {
                fn from(value: [<Db $type>]) -> Self {
                    Self::from(value.0)
                }
            }

            impl From<::sea_orm::entity::prelude::Uuid> for [<Db $type>] {
                fn from(value: ::sea_orm::entity::prelude::Uuid) -> Self {
                    Self(::uuid::Uuid::from_u128(value.as_u128()))
                }
            }

            impl ::sea_orm::TryFromU64 for [<Db $type>] {
                fn try_from_u64(n: u64) -> Result<Self, sea_orm::DbErr> {
                    Ok(::uuid::Uuid::from_u128(u128::from(n)).into())
                }
            }
        }
    };
}
