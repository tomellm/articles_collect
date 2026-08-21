use axum::extract::FromRef;
use leptos::{config::LeptosOptions, prelude::provide_context};
use sea_orm::DatabaseConnection;

/// Represents the whole state stored on the server. This should not contain
/// session specific stuff but only simple states, like repos and dependencies
/// or configs
#[derive(FromRef, Clone)]
pub struct ServerState {
    pub leptos_options: LeptosOptions,
    pub db: database::DbState,
}

impl ServerState {
    pub fn new(db: DatabaseConnection, leptos_options: LeptosOptions) -> Self {
        Self {
            db: db.into(),
            leptos_options,
        }
    }

    pub fn register_all_to_context(&self) {
        provide_context(self.clone());
        provide_context(self.db.tags.clone());
        provide_context(self.db.articles.clone());
    }
}
// crate::impl_from_ref!(ArticlesRepositoryImpl, input, &input.db.articles);
// crate::impl_from_ref!(TagsRepositoryImpl, input, &input.db.tags);
//
// #[macro_export]
// macro_rules! impl_from_ref {
//     ($type:ident, $var:ident, $access:expr) => {
//         impl ::axum::extract::FromRef<ServerState> for ::std::sync::Arc<$type> {
//             fn from_ref($var: &ServerState) -> Self {
//                 ::std::sync::Arc::clone($access)
//             }
//         }
//     };
// }
