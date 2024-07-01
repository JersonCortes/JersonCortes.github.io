use leptos::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="side-panel">
            <img src="https://avatars.githubusercontent.com/u/113752495?v=4" alt="Profile picture"></img>
            <div class="panel-text">
                <h3>Jerson Cortes P.</h3>
                <p>Software Developer</p>
            </div>
            <div class="icons">
	        <a href="https://www.linkedin.com/in/jerson-cortes-p" target="_blank">
    		    <img src="/assets/linkedin.svg" alt="LinkedIn"></img>
    		</a>
    		<a href="https://github.com/JersonCortes" target="_blank">
    		    <img src="/assets/github.svg" alt="Github"></img>
    		</a>
            </div>
        </div>
    }
}
