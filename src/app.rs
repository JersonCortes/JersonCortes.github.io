use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::routes::{
    home_page::HomePage,
    not_found::NotFound
};

use crate::components::side_bar::Sidebar;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/github-page.css"/>

        // sets the document title
        <Title text="Jerson Cortes P."/>

        // content for this welcome page
        <Router>
            <main>
                <div class="container">
                    <Sidebar/>
                    <div class="text-container">
                        <Routes fallback=|| NotFound>
                            <Route path=StaticSegment("") view=HomePage/>
                        </Routes>
    	              </div>
    	          </div>
            </main>
        </Router>
    }
}
