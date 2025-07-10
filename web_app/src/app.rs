use domain::articles::Article;
use leptos::prelude::*;
use leptos_keycloak_auth::components::ShowWhenAuthenticated;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::Router;

use crate::{
    keycloak::{InitAuth, LoginButton, UserNavBar},
    model::article::{get_articles, ArticlesList},
    routes,
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
        <Title text="Welcome to my Webiste"/>

        // content for this welcome page
        <Router>
            <InitAuth>
                <nav class="flex justify-between">
                    <h1 class="text-3xl">"Articles Collect"</h1>
                    <ShowWhenAuthenticated fallback=|| view!{<LoginButton />}>
                        <UserNavBar />
                    </ShowWhenAuthenticated>
                </nav>
                <main>
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
        <div class="flex justify-center bg-gray-500">
            <div class="max-w-[1000px] bg-white">
                <Suspense fallback=|| view! { "loading....." }>
                    {Suspend::new(async move {
                         let articles = articles_fn.await;
                         provide_context(articles.clone());
                         { view! { <ArticlesList articles/>}}
                     })}
                </Suspense>
            </div>
        </div>
    }
}

//<div class="w-full flex flex-col justify-center items-center">
//    <div class="max-w-2xl content-start p-6">
//        <div class="p-3 flex flex-col justify-center items-center">
//            <img class="w-70" src="./striezi.jpeg"/>
//        </div>
//        <h1 class="text-4xl text-left mb-3">"Striezi"</h1>
//        <h3 class="text-2xl text-left mb-2">"Strie|zi"</h3>
//        <p class="text-lg text-left">
//            "〈m.9; bayr.–österr.〉"
//            <span class="italic">"liebenswerter Lausbub, harmloser Gauner;"</span>
//            "<auch>"
//        </p>
//        <p class="text-lg text-left">
//            "〈österr.〉"
//            <span class="italic">"Strizzi"</span>
//        </p>
//    </div>
//</div>
