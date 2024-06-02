use leptos::*;
use leptos_meta::*;

#[component]
fn App() -> impl IntoView {

    let (count, set_count) = create_signal(0);

    view! {
        //<Meta name="color-scheme" content="dark"/>  

    }
}

fn main() {
    mount_to_body(|| {
        view! {
        }
    })
}
