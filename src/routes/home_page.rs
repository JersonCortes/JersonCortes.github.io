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
        <h2><span class="highlight">J</span>erson <span class="highlight">C</span>ortes <span class="highlight">P.</span></h2>
        <p>
            "New version coming soon."
        </p>

    }
}
