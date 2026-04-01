pub mod articles;
pub mod tags;

#[macro_export]
macro_rules! type_uuid {
    ($type:ident) => {
        #[derive(
            ::serde::Serialize, ::serde::Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash,
        )]
        #[repr(transparent)]
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

        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl ::std::str::FromStr for $type {
            type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($type::from(::uuid::Uuid::from_str(s)?))
            }
        }
    };
}
