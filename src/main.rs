use web_sys::{wasm_bindgen::{JsCast, UnwrapThrowExt}, HtmlTextAreaElement};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    let stored_notes = storage.get_item("notes").unwrap().unwrap_or_default();

    // Initialize default state with stored notes.
    let notes = use_state(|| stored_notes);

    // Handler for when the user types.
    let type_handler = Callback::from({
        let notes = notes.clone();
        move |input_event: InputEvent| {
            let target: HtmlTextAreaElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            notes.set(target.value());
            storage.set_item("notes", &target.value()).unwrap_throw();
        }
    });

    html! {
        <div>
            <textarea oninput={type_handler} value={(&*notes).clone()} spellcheck={"false"} />
            <style>
                {r#"
                    textarea {
                        width: 100%;
                        height: 100%;
                        font-size: 1.5rem;
                        padding: 0.5rem;
                        border: none;
                    }
                "#}
            </style>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
