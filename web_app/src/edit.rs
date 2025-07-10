use leptos::{form::MultiActionForm, prelude::*, server::ServerMultiAction};

use crate::{
    keycloak::Protected,
    model::article::AddArticles,
    utils::{Button, CenterColumn},
};

#[component]
pub fn EditArticles() -> impl IntoView {
    let add_articles = ServerMultiAction::<AddArticles>::new();

    view! {
        <CenterColumn>
            <div class="min-w-200">
                <Protected>
                    <MultiActionForm action=add_articles>
                        <div class="flex flex-col gap-2 p-2">
                            <div class="flex justify-between">
                                <label class="underline">
                                    "Upload List of Articles"
                                </label>
                                <Button>
                                    <input type="submit" value="Send"/>
                                </Button>
                            </div>
                            <textarea name="file_contents"
                                class="field-sizing-content h-100 border-1 border-gray-200">
                            </textarea>
                        </div>
                    </MultiActionForm>
                </Protected>
            </div>
        </CenterColumn>
    }
}
