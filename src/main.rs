use web_sys::{wasm_bindgen::{JsCast, UnwrapThrowExt}, HtmlTextAreaElement, HtmlInputElement};
use yew::prelude::*;

// ** TODO **
// Duplication cleanup

#[function_component]
fn App() -> Html {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();

    let stored_notes = storage.get_item("notes").unwrap().unwrap_or_default();
    let stored_bg_color = storage.get_item("bg_color").unwrap().unwrap_or("181926".to_string());
    let stored_text_color = storage.get_item("text_color").unwrap().unwrap_or("cad3f5".to_string());

    // Initialize default state with stored data.
    let notes = use_state(|| stored_notes);
    let bg_color = use_state(|| stored_bg_color);
    let text_color = use_state(|| stored_text_color);

    let set_storage = |key: &str, value: &str| {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        storage.set_item(key, value).unwrap_throw();
    };

    // Handler for when the user types.
    // TODO how to avoid this duplication
    let notes_type_handler = Callback::from({
        let notes = notes.clone();
        move |input_event: InputEvent| {
            let target: HtmlTextAreaElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();

            notes.set(target.value());
            set_storage("notes", &target.value());
        }
    });
    let bg_type_handler = Callback::from({
        let bg_color = bg_color.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();

            bg_color.set(target.value());
            set_storage("bg_color", &target.value());
        }
    });
    let text_type_handler = Callback::from({
        let text_color = text_color.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();

            text_color.set(target.value());
            set_storage("text_color", &target.value());
        }
    });

    let reset_handler = Callback::from({
        let bg_color = bg_color.clone();
        let text_color = text_color.clone();
        move |_| {
            set_storage("text_color", "cad3f5");
            text_color.set("cad3f5".to_string());

            set_storage("bg_color", "181926");
            bg_color.set("181926".to_string());
        }
    });

    html! {
        <div class="container">
            <div>
                <h1>{ "ZipNotes" }</h1>
                <input type="text" oninput={bg_type_handler} value={(&*bg_color).clone()} />
                <input type="text" oninput={text_type_handler} value={(&*text_color).clone()} />
                <button onclick={reset_handler}> { "reset colors" } </button>
            </div>
            <textarea oninput={notes_type_handler} value={(&*notes).clone()} spellcheck="false" />
            <style>
                {format!(r#"
                    body {{
                        background-color: #{};
                        color: #{};
                    }}
                    textarea {{
                        width: 100%;
                        height: 100vh;
                        font-size: 1.5rem;
                        border: none;
                        outline: none;
                        resize: none;
                        margin: 20px;
                        background: transparent;
                        color: #{};
                    }}
                    h1 {{
                        font-size: 3rem;
                        margin: 20px 0 0 20px;
                        font-family: monospace;
                    }}
                    input {{
                        font-size: 1.5rem;
                        /*border: none;*/
                        outline: none;
                        margin: 20px;
                        background: transparent;
                        font-family: monospace;
                        color: #{};
                    }}
                    button {{
                        font-size: 1.5rem;
                        outline: none;
                        margin: 20px;
                        background: transparent;
                        font-family: monospace;
                        color: #{};
                    }}
                "#, &*bg_color, &*text_color, &*text_color, &*text_color, &*text_color)}
            </style>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
