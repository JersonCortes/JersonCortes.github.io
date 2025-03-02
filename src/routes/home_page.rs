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
            "I am a software developer and data analyst with passion for open source. My expertise lies in full-stack web development with Rust, data analysis with Python, and low-level systems."
        </p>

    }
}
