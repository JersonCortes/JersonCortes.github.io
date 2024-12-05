<<<<<<< HEAD

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use github_page::app::*;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
=======
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
>>>>>>> source/main
}
