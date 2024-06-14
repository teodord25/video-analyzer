use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <NumericInput/> })
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something thats not)"
            <input type="number" on:input=on_input />


            // NOTE: we use this error boundary thing because Ok(T) is rendered
            // but Err(E) is just not rendered, so to show the error we use this
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Thats not a number"</p>
                        // we can render a list of errors as strings if we want
                        <ul>
                            { move || errors.get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered: "{value}</p>
            </ErrorBoundary>
        </label>
    }
}
