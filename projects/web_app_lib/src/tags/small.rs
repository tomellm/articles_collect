use domain::tags::ISmallTag;
use leptos::prelude::*;

/// Small tag component, just displaying the name of the tag.
#[component]
pub fn SmallTag(tag: impl ISmallTag) -> impl IntoView {
    view! {
        <div>{tag.name()}</div>
    }
}

/// Displays the tag name and the content of whatever the caller needs to be
/// shown next to the tag. The children will be shown to the right.
#[component]
pub fn TagWithAddition(tag: String, children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="border-2 py-1 px-2 flex gap-2">
            {tag.clone()}
            {move || children()}
        </div>
    }
}
