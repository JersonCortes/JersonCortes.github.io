use leptos::*;

#[component]
fn App() -> impl IntoView {

    view! {
        //<Meta name="color-scheme" content="dark"/>  
	<div class="header">
            	<div class="nav">
        		<a href="#">About</a>
    			<a href="#">Projects</a>
    			<a href="#">Contact</a>
    		</div>
    	</div>
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
                <p>"I am a software engineer who enjoys low level programming as well as understanding complex systems with 1 year of experience as a Data Analyst using Python with a European team. 

I'm interested on following my passion with low level programming. Since more than two years ago I've been using Linux in my home servers as well as for multiple projects. Since more than 2 years ago I commited to using Linux as my daily operating system. Right now I'm focused in c/c++ and Linux kernel development related opportunities."</p>
                <h2>Projects</h2>
                <hr></hr>
                
            </div>
    	</div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
