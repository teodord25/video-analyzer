use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
            Click me
        </button>

        <ProgressBar progress=count/>
        <ProgressBar progress=double_count/>
    }
}

#[component]
fn ProgressBar(
    #[prop(optional)] // optional means it can be omitted from the tag (but then the default will
    // be Default::default() for the type)

    #[prop(default = 10)] // you can set a default value for a prop
    max: u16,

    progress: impl Fn() -> i32 + 'static
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

// NOTE: you can use where clause
// #[component]
// fn ProgressBar2<F>(
//     #[prop(default = 0)]
//     max: u16,
//     progress: F
// ) -> impl IntoView
// where
//     F: Fn() -> i32 + 'static
// {
//     view! {
//         <progress
//             max=max
//             value=progress
//         />
//     }
// }


// NOTE: you can also inline generic
// #[component]
// fn ProgressBar3<F: Fn() -> i32 + 'static>(
//     #[prop(default = 0)]
//     max: u16,
//     progress: F
// ) -> impl IntoView {
//     view! {
//         <progress
//             max=max
//             value=progress
//         />
//     }
// }
