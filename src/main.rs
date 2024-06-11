use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let (x, set_x) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1)
            }

            class:red=move || count() % 2 == 1
        >
            "Click me: "
            // on stable, this is move || count.get();
            {count}
        </button>

        <button
            on:click=move |_| {
                set_x.update(|n| *n += 10)
            }

            style="position: absolute"

            style:left=move || format!("{}px", x() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            style=("--columns", x)
                >
            "Click to move"
        </button>

        <progress
                max="50"
                value=count
        />
    }
}
