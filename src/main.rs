use leptos::*;

mod components{
    pub mod sidebar;
    pub mod home_page;
}

use components::sidebar::Sidebar;
use components::home_page::Main;

#[component]
fn App() -> impl IntoView {
    view! {
    	<div class="container">
            <Sidebar></Sidebar>
            <div class="text-container">
                <Main></Main>
            </div>
    	</div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
