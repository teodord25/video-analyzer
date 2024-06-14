use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[component]
pub fn App() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: create_rw_signal(15),
        },
    ]);
    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value.update(|v| *v *= 2);
                }
            });
            // log the new value of the signal
            logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        // iterate over the rows and display each value
        <For
            each=data
            key=|state| state.key.clone()
            let:child
        >
            <p>{child.value}</p>
        </For>
    }
}

// #[component]
// fn App() -> impl IntoView {
//     let (count, set_count) = create_signal(0);
//     let double_count = move || count() * 2;
//
//     view! {
//         <button on:click=move |_| { set_count.update(|n| *n += 1); }>
//             Click me
//         </button>
//
//         <ProgressBar progress=count/>
//         <ProgressBar progress=double_count/>
//         <Iterator/>
//         <IteratorOfDynamicStuff/>
//         <DynamicList initial_length=3/>
//     }
// }
//
// /// A list of counters that allows you to add or
// /// remove counters.
// #[component]
// fn DynamicList(
//     /// The number of counters to begin with.
//     initial_length: usize,
// ) -> impl IntoView {
//     // This dynamic list will use the <For/> component.
//     // <For/> is a keyed list. This means that each row
//     // has a defined key. If the key does not change, the row
//     // will not be re-rendered. When the list changes, only
//     // the minimum number of changes will be made to the DOM.
//
//     // `next_counter_id` will let us generate unique IDs
//     // we do this by simply incrementing the ID by one
//     // each time we create a counter
//     let mut next_counter_id = initial_length;
//
//     // we generate an initial list as in <StaticList/>
//     // but this time we include the ID along with the signal
//     let initial_counters = (0..initial_length)
//         .map(|id| (id, create_signal(id + 1)))
//         .collect::<Vec<_>>();
//
//     // now we store that initial list in a signal
//     // this way, we'll be able to modify the list over time,
//     // adding and removing counters, and it will change reactively
//     let (counters, set_counters) = create_signal(initial_counters);
//
//     let add_counter = move |_| {
//         // create a signal for the new counter
//         let sig = create_signal(next_counter_id + 1);
//         // add this counter to the list of counters
//         set_counters.update(move |counters| {
//             // since `.update()` gives us `&mut T`
//             // we can just use normal Vec methods like `push`
//             counters.push((next_counter_id, sig))
//         });
//         // increment the ID so it's always unique
//         next_counter_id += 1;
//     };
//
//     view! {
//         <div>
//             <button on:click=add_counter>
//                 "Add Counter"
//             </button>
//             <ul>
//                 // The <For/> component is central here
//                 // This allows for efficient, key list rendering
//                 <For
//                     // `each` takes any function that returns an iterator
//                     // this should usually be a signal or derived signal
//                     // if it's not reactive, just render a Vec<_> instead of <For/>
//                     each=counters
//                     // the key should be unique and stable for each row
//                     // using an index is usually a bad idea, unless your list
//                     // can only grow, because moving items around inside the list
//                     // means their indices will change and they will all rerender
//                     key=|counter| counter.0
//                     // `children` receives each item from your `each` iterator
//                     // and returns a view
//                     children=move |(id, (count, set_count))| {
//                         view! {
//                             <li>
//                                 <button
//                                     on:click=move |_| set_count.update(|n| *n += 1)
//                                 >
//                                     {count}
//                                 </button>
//                                 <button
//                                     on:click=move |_| {
//                                         set_counters.update(|counters| {
//                                             counters.retain(|(counter_id, (signal, _))| {
//                                                 // NOTE: in this example, we are creating the signals
//                                                 // in the scope of the parent. This means the memory used to
//                                                 // store them will not be reclaimed until the parent component
//                                                 // is unmounted. Here, we're removing the signal early (i.e, before
//                                                 // the DynamicList is unmounted), so we manually dispose of the signal
//                                                 // to avoid leaking memory.
//                                                 //
//                                                 // This is only necessary in an example with nested signals like this one.
//                                                 if counter_id == &id {
//                                                     signal.dispose();
//                                                 }
//                                                 // NOTE: this will remove the li in which the
//                                                 // button is clicked because this will only evaluate to
//                                                 // false for that li, and retain will keep all the other lis
//                                                 counter_id != &id
//                                             })
//                                         });
//                                     }
//                                 >
//                                     "Remove"
//                                 </button>
//                             </li>
//                         }
//                     }
//                 />
//             </ul>
//         </div>
//     }
// }
//
// /// Shows progress towards a goal.
// #[component]
// fn ProgressBar(
//     /// The maximum value of the progress bar.
//     #[prop(default = 10)] // you can set a default value for a prop
//     max: u16,
//
//     /// How much progress has been made.
//     progress: impl Fn() -> i32 + 'static,
// ) -> impl IntoView {
//     view! {
//         <progress
//             max=max
//             value=progress
//         />
//     }
// }
//
// #[component]
// fn Iterator() -> impl IntoView {
//     let values = vec![0, 1, 2];
//     view! {
//         // this will just render "012"
//         <p>{values.clone()}</p>
//         // or we can wrap them in <li>
//         <ul>
//             {values.into_iter()
//                 .map(|n| view! { <li>{n}</li>})
//                 .collect_view()} // convenient helper func eq to .collect::<Vec<_>>()
//         </ul>
//     }
// }
//
// #[component]
// fn IteratorOfDynamicStuff() -> impl IntoView {
//     let length = 5;
//     let counters = (1..length).map(|idx| create_signal(idx));
//
//     let counter_buttons = counters
//         .map(|(count, set_count)| {
//             view! {
//                 <li>
//                     <button
//                         on:click=move |_| set_count.update(|n| *n += 1)
//                     >
//                         {count}
//                     </button>
//                 </li>
//             }
//         })
//         .collect_view();
//
//     view! {
//         <ul>
//             {counter_buttons}
//         </ul>
//     }
// }
//
// // NOTE: into() approach
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
