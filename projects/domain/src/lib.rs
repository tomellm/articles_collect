pub mod articles;
pub mod tags;

#[macro_export]
macro_rules! type_uuid {
    ($type:ident) => {
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, Debug, Eq, PartialEq)]
        pub struct $type(::uuid::Uuid);

        impl $type {
            pub fn new() -> Self {
                Self(::uuid::Uuid::new_v4())
            }
        }

        impl From<::uuid::Uuid> for $type {
            fn from(value: ::uuid::Uuid) -> Self {
                Self(value)
            }
        }

        impl Default for $type {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        impl ::std::ops::Deref for $type {
            type Target = ::uuid::Uuid;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
