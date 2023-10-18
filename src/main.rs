use leptos::*;

fn main() {
    // mount_to_body(|| view! { <p>"ハロー・ワールド"</p> })
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (num1, set_num1) = create_signal(0);
    let (num2, set_num2) = create_signal(0);

    view! {
        <p>
            "Num 1: "
            <input
                type="number"
                value=num1.get()
                on:change=move |e| set_num1.update(|v| *v = event_target_value(&e).parse().unwrap())
            />
        </p>
        <p>
            "Num 2: "
            <input
                type="number"
                value=num2.get()
                on:change=move |e| set_num2.update(|v| *v = event_target_value(&e).parse().unwrap())
            />
        </p>
        <p>
            {move || num1.get().to_string()}
            " ＋ "
            {move || num2.get().to_string()}
            " ＝ "
            {move || num1.get() + num2.get()}
        </p>
    }
}
