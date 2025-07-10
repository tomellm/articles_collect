use leptos::prelude::*;

#[component]
pub fn CenterColumn(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <div class="max-w-[1000px] bg-white outline-2">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn Button(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="flex justify-content items-center border-2 py-1 px-2
            hover:bg-black hover:text-white 
            hover:border-l-gray-200 hover:border-b-gray-200
            hover:border-t-gray-950 hover:border-r-gray-950
            ">
            { children() }
        </div>
    }
}
