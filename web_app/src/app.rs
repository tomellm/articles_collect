use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_oidc::AuthSignal;
use leptos_router::{
    components::{Route, Router, Routes, A},
    path,
};
use tracing::info;

use crate::{
    articles::{edit::EditArticles, list::ArticlesList},
    keycloak::{InitAuth, KeycloakInfo, LoginButton, Logout, ShowWhenAuthenticated},
    routes::FallbackRoute,
    utils::{Button, CenterColumn, DialogSignal},
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
                        <Route path=path!("/edit") view=EditArticles />
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

#[component]
fn GlobalNavBar() -> impl IntoView {
    let nav_open = RwSignal::new(false);

    let auth = expect_context::<AuthSignal>();

    info!("{:?}", auth.get());
    view! {
        <div class="fixed top-0 left-0 z-10 w-screen"
            class:h-screen=move || nav_open.get()>
            <nav class="flex gap-4 py-2 px-5 border-2 w-full
                bg-white items-center"
                class:flex-col=move || nav_open.get()
                class:h-screen=move || nav_open.get()>
                <div class="flex justify-between w-full h-20 items-center">
                    <h1 class="text-3xl">"Articles Collect"</h1>
                    <button
                          on:click=move |_| nav_open.set(!nav_open.get())>
                        <Button>
                            {move || match nav_open.get() {
                                true => "^",
                                false => "v"
                            }}
                        </Button>
                    </button>
                </div>

                <div class="flex flex-col"
                    class:hidden=move || !nav_open.get()
                    class:block=move || nav_open.get()>
                    <ShowWhenAuthenticated fallback=|| view!{<LoginButton />}>
                        <div>
                            <A href="/edit" on:click=move |_| nav_open.set(false)>
                                "Add Articles"
                            </A>
                            <Logout />
                        </div>
                    </ShowWhenAuthenticated>
                </div>
            </nav>
        </div>
    }
}

#[component]
fn GlobalDialog(mut dialog: DialogSignal) -> impl IntoView {
    view! {
        <dialog open=move || dialog.is_open()
            class:block=move || dialog.is_open()
            class="bg-black/40 z-200 border-box fixed">
            <div class=" grid place-content-center w-screen h-screen">
                <form method="dialog" class="bg-white">
                    <div class="m-5">
                        <div class="flex justify-between">
                            <h2 class="text-xl">{ move || dialog.title_text() }</h2>
                            <button on:click=move |_| dialog.close()>
                                <Button>
                                    "x"
                                </Button>
                            </button>
                        </div>
                        <h3 class="py-2 text-xl">
                            { move || dialog.content_text() }
                        </h3>
                        <div class="flex justify-between">
                            <button on:click=move |_| dialog.no_action()>
                                <Button>
                                    { move || dialog.no_text() }
                                </Button>
                            </button>
                            <button on:click=move |_| dialog.yes_action()>
                                <Button>
                                    { move || dialog.yes_text() }
                                </Button>
                            </button>
                        </div>
                    </div>
                </form>
            </div>
        </dialog>
    }
}
