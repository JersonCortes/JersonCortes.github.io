use leptos::*;

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>About me</h2>
        <hr/>
        <p>
            "I am a software engineer who enjoys low level programming as well as understanding complex systems with 1 year of experience as a Data Analyst using Python with a European team. I'm interested on following my passion with low level programming. Since more than two years ago I've been using Linux in my home servers as well as for multiple projects. Since more than 2 years ago I commited to using Linux as my daily operating system. Right now I'm focused in c/c++ and Linux kernel development related opportunities."
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

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <About/>
        <Projects/>
        <Recent_blogs/>
    }
}
