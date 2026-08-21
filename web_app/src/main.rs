#![allow(non_snake_case)]
#![allow(async_fn_in_trait)]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    web_app_lib::server_start().await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
