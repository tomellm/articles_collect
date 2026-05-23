use domain::tags::TagUuid;
use leptos::prelude::*;

use crate::utils::Button;

#[component]
pub fn EditOrAddTagForm(
    uuid: RwSignal<Option<TagUuid>>,
    name: RwSignal<String>,
    description: RwSignal<String>,
    new_result: Action<bool, ()>,
    submit_text: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            { move || uuid.get().map(|uuid| view! {
                <div>
                    <label class="pr-5">
                        "Uuid:"
                    </label>
                    <input type="text" name="tag_uuid"
                        value=move || uuid.to_string()
                        readonly
                        class="border-1 border-gray-200 field-sizing-content w-md"/>
                </div>
            })}
            <div class="flex justify-between">
                <div>
                    <label class="pr-5">
                        "Name:"
                    </label>
                    <input type="text" name="tag_name"
                        bind:value=name
                        placeholder="your tag name..."
                        class="border-1 border-gray-200 field-sizing-content "/>
                </div>
                <div class="flex gap-2">
                    <Button on_click=move || { new_result.dispatch(true); }>
                        "Reset"
                    </Button>
                    <Button button_type="submit">
                        {submit_text}
                    </Button>
                </div>
            </div>
            <label>
                "Description:"
            </label>
            <textarea name="tag_description"
                placeholder="your tag description..."
                bind:value=description
                class="field-sizing-content h-30 border-1 border-gray-200">
            </textarea>
        </div>
    }
}
