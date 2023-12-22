use web_sys::{wasm_bindgen::{JsCast, UnwrapThrowExt}, HtmlTextAreaElement, HtmlInputElement};
use yew::prelude::*;

// ** TODO **
// Make textarea resizeable (make it resize as more text is written)
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
        <div style="overflow-x: hidden;">
            <div>
                <div class="flex">
                <h1>{ "ZipNotes" }</h1>
                <a href="https://github.com/jonasiwnl/zipnotes" target="_blank">
                    <svg xmlns="http://www.w3.org/2000/svg"  viewBox="0 0 50 50" width="40px" height="40px">
                        <path fill={format!("#{}", (&*text_color))} d="M17.791,46.836C18.502,46.53,19,45.823,19,45v-5.4c0-0.197,0.016-0.402,0.041-0.61C19.027,38.994,19.014,38.997,19,39 c0,0-3,0-3.6,0c-1.5,0-2.8-0.6-3.4-1.8c-0.7-1.3-1-3.5-2.8-4.7C8.9,32.3,9.1,32,9.7,32c0.6,0.1,1.9,0.9,2.7,2c0.9,1.1,1.8,2,3.4,2 c2.487,0,3.82-0.125,4.622-0.555C21.356,34.056,22.649,33,24,33v-0.025c-5.668-0.182-9.289-2.066-10.975-4.975 c-3.665,0.042-6.856,0.405-8.677,0.707c-0.058-0.327-0.108-0.656-0.151-0.987c1.797-0.296,4.843-0.647,8.345-0.714 c-0.112-0.276-0.209-0.559-0.291-0.849c-3.511-0.178-6.541-0.039-8.187,0.097c-0.02-0.332-0.047-0.663-0.051-0.999 c1.649-0.135,4.597-0.27,8.018-0.111c-0.079-0.5-0.13-1.011-0.13-1.543c0-1.7,0.6-3.5,1.7-5c-0.5-1.7-1.2-5.3,0.2-6.6 c2.7,0,4.6,1.3,5.5,2.1C21,13.4,22.9,13,25,13s4,0.4,5.6,1.1c0.9-0.8,2.8-2.1,5.5-2.1c1.5,1.4,0.7,5,0.2,6.6c1.1,1.5,1.7,3.2,1.6,5 c0,0.484-0.045,0.951-0.11,1.409c3.499-0.172,6.527-0.034,8.204,0.102c-0.002,0.337-0.033,0.666-0.051,0.999 c-1.671-0.138-4.775-0.28-8.359-0.089c-0.089,0.336-0.197,0.663-0.325,0.98c3.546,0.046,6.665,0.389,8.548,0.689 c-0.043,0.332-0.093,0.661-0.151,0.987c-1.912-0.306-5.171-0.664-8.879-0.682C35.112,30.873,31.557,32.75,26,32.969V33 c2.6,0,5,3.9,5,6.6V45c0,0.823,0.498,1.53,1.209,1.836C41.37,43.804,48,35.164,48,25C48,12.318,37.683,2,25,2S2,12.318,2,25 C2,35.164,8.63,43.804,17.791,46.836z"/>
                    </svg>
                </a>
                </div>
                <label style="margin: 20px;">
                    { "bg" }
                    <input type="text" oninput={bg_type_handler} value={(&*bg_color).clone()} />
                </label>
                <label>
                    {" text "}
                    <input type="text" oninput={text_type_handler} value={(&*text_color).clone()} />
                </label>
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
                        height: 300vh;
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
                        outline: none;
                        margin: 20px;
                        background: transparent;
                        font-family: monospace;
                        color: #{};
                    }}
                    label {{
                        font-family: monospace;
                        font-size: 1.5rem;
                    }}
                    button {{
                        font-size: 1.5rem;
                        outline: none;
                        margin: 20px;
                        background: transparent;
                        font-family: monospace;
                        color: #{};
                    }}
                    svg {{
                        margin: 20px 20px 0 0;
                    }}
                    .flex {{
                        display: flex;
                        justify-content: space-between;
                    }}
                "#, &*bg_color, &*text_color, &*text_color, &*text_color, &*text_color)}
            </style>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
