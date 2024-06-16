use leptos::*;
use leptos::ev::MouseEvent;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

// child -> parent communication
//
// 2. passing a callback from parent to child
#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        // you pass the callback to the child component
        <ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
    }
}


#[component]
pub fn ButtonB(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView
{
    view! {
        <button on:click=on_click> // the child click event is only used to execute the on_click
        // function that was passed
            "Toggle"
        </button>
    }
}
