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

/// Shows progress towards a goal.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 10)] // you can set a default value for a prop
    max: u16,

    /// How much progress has been made.
    progress: impl Fn() -> i32 + 'static
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

// NOTE: into() approach
// #[component]
// fn ProgressBar4(
//     #[prop(default = 10)]
//     max: u16,
//
//     // .into() converts `ReadSignal` to `Signal`
//     // <ProgressBar progress=count/>
//     // use `Signal::derive()` to wrap a derived signal
//     // <ProgressBar progress=Signal::derive(double_count)/>]
//
//     #[prop(into)
//     progress: Signal<i32>
// ) -> impl IntoView {
//     view! {
//         <progress
//             max=max
//             value=progress
//         />
//     }
// }


// NOTE: you can use where clause
// #[component]
// fn ProgressBar2<F>(
//     #[prop(default = 10)]
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
//     #[prop(default = 10)]
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
