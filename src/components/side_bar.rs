use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="side-panel">
            <img src="https://avatars.githubusercontent.com/u/113752495?v=4" alt="Profile picture"/>
            <div class="panel-text">
                <h3>Jerson Cortes P.</h3>
                <p>Software Developer</p>
            </div>
            <div class="icons">
                <a href="https://www.linkedin.com/in/jerson-cortes-p" target="_blank">
                    <img src="/public/linkedin.svg" alt="LinkedIn"/>
                </a>
                <a href="https://github.com/JersonCortes" target="_blank">
                    <img src="/public/github.svg" alt="Github"/>
                </a>
            </div>
        </div>
    }
}
