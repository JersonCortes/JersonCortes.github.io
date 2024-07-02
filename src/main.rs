use leptos::*;

mod components{
    pub mod sidebar;
    pub mod home_page;
    pub mod blog;
}

use components::sidebar::Sidebar;
use components::home_page::Main;
use components::blog::Blog;

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = create_signal(true);

    view! {
    	<div class="container">
            <Sidebar/>
            <div class="text-container">
                {move || if value(){
                    view! {<Main/>}
                } else {
                    view! {<Blog/>}
                }}
            </div>
    	</div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
