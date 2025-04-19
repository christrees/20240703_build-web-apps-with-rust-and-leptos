use leptos::{ev::MouseEvent, *};

#[component]
pub fn Child(
    counter: ReadSignal<i16>,
    #[prop(into)] on_increment: Callback<MouseEvent>,
    #[prop(into)] on_decrement: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <div>
            <div style="border: 1px solid black; padding: 10px;">
                <h3>"Child Callback"</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button on:click=on_increment>"callback Increment"</button>
                    <button on:click=on_decrement>"callback Decrement"</button>
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
                <h3>"Parent Callback"</h3>
                <p>"Counter: " {counter}</p>
                <div>
                    <button on:click=increment_counter>"Increment"</button>
                    <button on:click=decrement_counter>"Decrement"</button>
                </div>
                <p>"Click the button to increment or decrement the counter."</p>
                <p>"The counter value is passed to the child component."</p>
            </div>
            <Child
                counter=counter
                on_increment=increment_counter
                on_decrement=decrement_counter />
        </div>
    }
}
