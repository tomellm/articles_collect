use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Routes},
    path,
};

use crate::{
    app::HomePage,
    edit::EditArticles,
    keycloak::{InitAuth, Logout},
};

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
