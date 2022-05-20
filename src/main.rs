use yew::prelude::*;

struct Counter {
    value: i32
}

#[function_component(App)]
fn app() -> Html {

    let state = use_state(|| Counter {
        value: 0
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Counter {
                value: state.value + 1
            })
        })
    };

    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <p> {state.value} </p>
        </div>
    }
    
}

fn main() {

    yew::start_app::<App>();
}
