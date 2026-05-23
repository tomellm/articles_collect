use leptos::prelude::*;

const BUTTON_CSS_CLASSES: &str = "flex justify-content items-center border-2 py-1 px-2
            hover:bg-black hover:text-white bg-white
            hover:border-l-gray-200 hover:border-b-gray-200
            hover:border-t-gray-950 hover:border-r-gray-950
            text-xl md:text-base";

/// The same as the normal Button class just that it does not actually contain
/// a button tag, allowing the user to use the same style for other things
#[component]
pub fn NoButton(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class=BUTTON_CSS_CLASSES>
            { children() }
        </div>
    }
}

/// A component to visually style buttons. Already contains a button tag
/// internally, meaning the onclick event should be passed to
#[component]
pub fn Button(
    #[prop(optional, into)] on_click: Option<Callback<(), ()>>,
    #[prop(default = "button")] button_type: &'static str,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div class=BUTTON_CSS_CLASSES>
            <button on:click=move |_|{ if let Some(f) = on_click { f.run(()) } }
                type=button_type
                class="-my-1 -mx-2 py-1 px-2">
                { children() }
            </button>
        </div>
    }
}
