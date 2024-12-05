use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <About/>
        <Projects/>
        <Recent_blogs/>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>About me</h2>
        <hr/>
        <p>
            "Software developer and Data analyst."
        </p>

    }
}

#[component]
fn Projects() -> impl IntoView {
    view! {
        <h2>Projects</h2>
        <hr/>
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
    }
}

#[component]
fn Recent_blogs() -> impl IntoView {
    view! {
        <h2>Recent blogs</h2>
        <hr/>
        <h3>Kernel Development</h3>
        <p>
            Working on my first kernel patch.
        </p>
    }
}
