use leptos::*;

mod components{
    pub mod sidebar;
    pub mod home_page;
    pub mod blog;
}

use components::{
    blog::Blog, 
    home_page::Main, 
    sidebar::Sidebar
};

#[component]
fn App() -> impl IntoView {
    let (value, _set_value) = create_signal(false);

    view! {
        <div class="container">
            <Sidebar/>
            <div class="text-container">
                <Main/>
                //{move || if value(){
                //    view! {<Main/>}
                //} else {
                //    view! {<Blog/>}
                //}}
            </div>
    	  </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
