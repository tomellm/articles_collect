use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Routes},
    path,
};
use tracing::info;

use crate::{
    app::HomePage,
    edit::EditArticles,
    keycloak::{InitAuth, Logout},
};

pub fn routes() -> impl IntoView {
    info!("routes");
    view! {
        <Routes fallback=FallbackRoute>
            <ParentRoute path=path!("") view=InitAuthParent>
                <Route path=path!("") view=HomePage />
                <Route path=path!("edit") view=EditArticles />
                <Route path=path!("logout") view=Logout />
            </ParentRoute>
        </Routes>
    }
}

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
