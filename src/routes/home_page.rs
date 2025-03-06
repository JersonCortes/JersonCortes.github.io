use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <About/>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>Jerson Cortes P.</h2>
        <p>
            "New version coming soon."
        </p>

    }
}
