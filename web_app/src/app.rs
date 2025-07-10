use leptos::prelude::*;
use leptos_keycloak_auth::components::ShowWhenAuthenticated;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::Router;

use crate::{
    keycloak::{InitAuth, LoginButton, UserNavBar},
    model::article::{get_articles, ArticlesList},
    routes,
    utils::CenterColumn,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web_app.css"/>

        // sets the document title
        <Title text="cool articles [here!]"/>

        // content for this welcome page
        <Router>
            <InitAuth>
                <nav class="flex justify-between py-2 px-5 border-2 fixed w-full h-20 bg-white items-center">
                    <h1 class="text-3xl">"Articles Collect"</h1>
                    <ShowWhenAuthenticated fallback=|| view!{<LoginButton />}>
                        <UserNavBar />
                    </ShowWhenAuthenticated>
                </nav>
                <main class="pt-20">
                    { routes::routes() }
                </main>
            </InitAuth>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let articles_fn = OnceResource::new(async { get_articles().await.unwrap() });
    view! {
        <CenterColumn>
            <Suspense fallback=|| view! { "loading....." }>
                {Suspend::new(async move {
                     let articles = articles_fn.await;
                     provide_context(articles.clone());
                     { view! { <ArticlesList articles/>}}
                 })}
            </Suspense>
        </CenterColumn>
    }
}
