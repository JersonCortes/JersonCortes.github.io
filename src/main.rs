use leptos::*;

#[component]
fn App() -> impl IntoView {

    view! {
    <div class="container">

        <h1>"Welcome to my portfolio!"</h1>
        <h2>"On Github Pages"</h2>

    </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
