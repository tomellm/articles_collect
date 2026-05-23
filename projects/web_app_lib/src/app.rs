use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

use crate::{
    articles::{edit::EditArticles, list::ArticlesList, single::SingleArticle},
    keycloak::{InitAuth, KeycloakInfo, LoginButton, Logout, ShowWhenAuthenticated},
    routes::FallbackRoute,
    tags::edit::EditTags,
    utils::{
        Button, CenterColumn,
        dialog::{DialogSignal, GlobalDialog},
    },
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
    let _keycloak_info = SharedValue::new(KeycloakInfo::from_env);

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let dialog = {
        let dialog = DialogSignal::default();
        provide_context(dialog);
        dialog
    };

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web_app.css"/>

        // sets the document title
        <Title text="cool articles [here!]"/>

        // content for this welcome page
        <GlobalDialog dialog />
        <main class="">
            <Router>
                <InitAuth>
                    <GlobalNavBar />
                    <Routes fallback=FallbackRoute>
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/articles") view=HomePage />
                        <Route path=path!("/articles/:uuid") view=SingleArticle />
                        <Route path=path!("/edit") view=EditArticles />
                        <Route path=path!("/tags") view=EditTags />
                    </Routes>
                </InitAuth>
            </Router>
        </main>
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <CenterColumn>
            <ArticlesList />
        </CenterColumn>
    }
}

/// The global navbar that allows for naviation between pages and other actions
/// like login and logout
#[component]
fn GlobalNavBar() -> impl IntoView {
    let nav_open = RwSignal::new(false);

    view! {
        <div class="fixed top-0 left-0 z-10 w-screen"
            class:h-screen=move || nav_open.get()>
            <nav class="flex gap-4 py-2 px-5 border-2 w-full
                bg-white items-center"
                class:flex-col=move || nav_open.get()
                class:h-screen=move || nav_open.get()>
                <div class="flex justify-between w-full h-20 items-center">
                    <A href="/" on:click=move |_| nav_open.set(false)>
                      <h1 class="text-3xl">"Articles Collect"</h1>
                    </A>
                    <Button on_click=move || nav_open.set(!nav_open.get())>
                        {move || match nav_open.get() {
                            true => "^",
                            false => "v"
                        }}
                    </Button>
                </div>

                <div class="flex flex-col items-center gap-2 text-2xl"
                    class:hidden=move || !nav_open.get()
                    class:block=move || nav_open.get()>
                    <ShowWhenAuthenticated fallback=|| view!{<LoginButton />}>
                        <div class="flex flex-col items-center gap-4">
                            <A href="/edit" on:click=move |_| nav_open.set(false)>
                                "Add Articles"
                            </A>
                            <A href="/tags" on:click=move |_| nav_open.set(false)>
                                "Edit Tags"
                            </A>
                            <Logout />
                        </div>
                    </ShowWhenAuthenticated>
                </div>
            </nav>
        </div>
    }
}
