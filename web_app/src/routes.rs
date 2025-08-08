use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::keycloak::InitAuth;

#[component]
pub fn InitAuthParent() -> impl IntoView {
    view! {
        <InitAuth>
            <Outlet />
        </InitAuth>
    }
}

#[component]
pub fn FallbackRoute() -> impl IntoView {
    view! {
        "Page not Found"
    }
}
