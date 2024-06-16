use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

// child -> parent communication
//
// 3. using an event listener if the callback maps to a native dom element
#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonB on:click=move |_| set_toggled.update(|value| *value = !*value)/>
        // using on:click now (event listener) instead of on_click (callback arg)
    }
}


#[component]
pub fn ButtonB() -> impl IntoView {
    view! {
        <button> "Toggle" </button>
    }
}
