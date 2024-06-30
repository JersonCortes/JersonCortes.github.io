use leptos::*;

#[component]
fn App() -> impl IntoView {

    view! {
        //<Meta name="color-scheme" content="dark"/>  
    	<div class="container">
            <div class="side-panel">
                <img src="https://via.placeholder.com/150" alt="Sample Image"></img>
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
            <div class="text-container">
                <h2>About me</h2>
                <hr></hr>
                <p>"I am a software engineer who enjoys low level programming as well as understanding complex systems with 1 year of experience as a Data Analyst using Python with a European team. I'm interested on following my passion with low level programming. Since more than two years ago I've been using Linux in my home servers as well as for multiple projects. Since more than 2 years ago I commited to using Linux as my daily operating system. Right now I'm focused in c/c++ and Linux kernel development related opportunities."</p>
                <h2>Projects</h2>
                <hr></hr>
                <a href="https://github.com/JersonCortes/telescope-podman">
                    <h3>Container.Telescope.nvim</h3>
                </a>
                <p>
                    A Neovim plugin to integrate container into development by using Telescope that currently works with Podman.
                </p>
                <p>
                    This project was born out the need of developers to have reproducible environments for ease of development and collaboration. With increasing recognition and support for devcontainers Neovim falls behind in integration of them into the ecosystem compared to other IDEs.
                </p>
                <p>
                    The goal of this project is to provide a plugin that removes the tedious tasks of using containers for development. In the future this plugin should: Recognize your working environment, show relevant information of the containers you are working with, create relevant container files, work with OCI compliant engines.
                </p>

                <h2>Interests</h2>
                <hr></hr>
                <h2>Skills</h2>
                <hr></hr>
                <h4>C</h4>
                <h4>C++</h4>
                <h4>Rust</h4>
                <h2>Currently Learning</h2>
                <hr></hr>

            </div>
    	</div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
