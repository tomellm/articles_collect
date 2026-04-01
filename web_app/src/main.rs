#![allow(non_snake_case)]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    web_app::server_start().await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
