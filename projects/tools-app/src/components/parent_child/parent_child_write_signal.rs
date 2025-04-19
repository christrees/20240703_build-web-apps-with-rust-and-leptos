use leptos::*;

#[component]
pub fn Child(counter: ReadSignal<i16>, set_counter: WriteSignal<i16>) -> impl IntoView {
    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);
    view! {
        <div>
            <div style="border: 1px solid black; padding: 4px;">
                <h3>"Child Write Signal"</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button on:click=increment_counter>"child Increment"</button>
                    <button on:click=decrement_counter>"child Decrement"</button>
                </div>
                <p>"Click the button to increment or decrement the counter."</p>
                <p>"The counter value is passed to the child component."</p>
            </div>
        </div>
    }
}

#[component]
pub fn Parent() -> impl IntoView {
    let (counter, set_counter) = create_signal::<i16>(0);
    let increment_counter = move |_| set_counter.update(|c| *c += 1);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 1);
    view! {
        <div>
            <div style="border: 1px solid black; padding: 10px;">
                <h3>"Parent Write Signal"</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button on:click=increment_counter>"Increment"</button>
                    <button on:click=decrement_counter>"Decrement"</button>
                </div>
                <p>"Click the button to increment or decrement the counter."</p>
                <p>"The counter value is passed to the child component."</p>
            </div>
            <Child counter=counter set_counter=set_counter />
        </div>
       
    }
}