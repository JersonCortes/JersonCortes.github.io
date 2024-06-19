use leptos::*;

#[component]
fn App() -> impl IntoView {

    view! {
        //<Meta name="color-scheme" content="dark"/>  
	<div class="header">
        	<div class="inter title">
                        Jerson Cortes P.
                </div>
         	<div class="nav">
        		<a href="#">About</a>
    			<a href="#">Projects</a>
    			<a href="#">Contact</a>
    		</div>
    	</div>
    	<div class="main">
            <div class="sidebar-profile">

            </div>
            <div class="info">

            </div>
    	</div>
    	<div class="footer">
    		<a href="https://www.linkedin.com/in/jerson-cortes-p" target="_blank">
    			<img src="/assets/linkedin.svg" alt="LinkedIn"></img>
    		</a>
    		<a href="https://github.com/JersonCortes" target="_blank">
    			<img src="/assets/github.svg" alt="Github"></img>
    		</a>
    	</div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
