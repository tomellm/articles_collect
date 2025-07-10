use leptos::{form::MultiActionForm, prelude::*, server::ServerMultiAction};

use crate::{keycloak::Protected, model::article::AddArticles};

#[component]
pub fn EditArticles() -> impl IntoView {
    let add_articles = ServerMultiAction::<AddArticles>::new();

    view! {
        <Protected>
            <MultiActionForm action=add_articles>
                <div class="flex flex-col">
                    <label>
                        "Upload List of Articles"
                    </label>
                    <textarea name="file_contents">
                    </textarea>
                    <input type="submit" value="Send"/>
                </div>
            </MultiActionForm>
        </Protected>
    }
}
