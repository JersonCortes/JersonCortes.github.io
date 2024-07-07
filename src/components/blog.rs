use leptos::*;

#[component]
pub fn Blog() -> impl IntoView {
    view!{
        <h2>Blogs</h2>
        <hr/>
        <Entry/>
    }
}

#[component]
fn Entry() -> impl IntoView {
    view! {
        <h3>Kernel Development</h3>
        <p>
            Working on my first kernel patch.
        </p>
    }
}
