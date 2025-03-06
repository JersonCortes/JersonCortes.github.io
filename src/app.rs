use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::routes::{
    home_page::HomePage,
    not_found::NotFound
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet href="/public/style.css"/>

        <Title text="Jerson Cortes P."/>

        <Router>
            <main>
                <div class="container">
                    <Routes fallback=|| NotFound>
                        <Route path=StaticSegment("") view=HomePage/>
                    </Routes>
    	          </div>
            </main>
        </Router>
    }
}
