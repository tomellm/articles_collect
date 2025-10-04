use leptos::prelude::*;

use crate::utils::Loader;

pub trait BusyAction {
    fn is_busy(&self) -> bool;
    fn busy_text(&self) -> String;
}

impl<T> BusyAction for Option<T>
where
    T: BusyAction,
{
    fn is_busy(&self) -> bool {
        match self {
            Some(state) => state.is_busy(),
            None => false,
        }
    }

    fn busy_text(&self) -> String {
        match self {
            Some(state) => state.busy_text(),
            None => String::new(),
        }
    }
}

#[component]
pub fn BusyContainer<T>(busy_state: T, children: ChildrenFn) -> impl IntoView
where
    T: BusyAction + Copy + Send + Sync + 'static,
{
    view! {
        <div class="relative">
            <Show when=move || busy_state.is_busy()>
                <div class="absolute top-0 left-0 right-0 h-full bg-blend-lighten bg-white/75 place-content-center">
                    <div class="flex flex-col justify-center items-center">
                        <Loader />
                        <h3 class="text-xl text-gray-500">
                            { move || busy_state.busy_text() }
                        </h3>
                    </div>
                </div>
            </Show>
            { children() }
        </div>
    }
}
