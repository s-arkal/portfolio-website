use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::sidebar_left::SidebarLeft;
use crate::pages::contact::ContactPage;
use crate::pages::background::BackgroundPage;
use crate::pages::resume::ResumePage;
use crate::pages::home::HomePage;
use crate::pages::projects::ProjectsPage;
use crate::components::sidebar_right::SidebarRight;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rust-leptos-portfolio.css" />
        <Stylesheet id="devicon" href="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/devicon.min.css" />

        <Title text="Shriyans Arkal — Software Engineer" />
        <Meta name="description" content="Technical portfolio of Shriyans Arkal. Software Engineer specializing in Agentic AI." />
        <Meta name="author" content="Shriyans Arkal" />

        <Router>
            <div class="min-h-screen bg-bg text-txt font-mono flex flex-col md:flex-row h-screen overflow-hidden">
                // left navigation
                <SidebarLeft />

                // main content
                <main class="flex-1 overflow-y-auto px-4 py-8 sm:px-8 sm:py-12 lg:px-16 lg:py-20 relative bg-bg">
                    <div class="max-w-4xl mx-auto">
                        <Routes fallback=|| view! { <NotFound /> }>
                            <Route path=path!("/") view=HomePage />
                            <Route path=path!("/projects") view=ProjectsPage />
                            <Route path=path!("/background") view=BackgroundPage />
                            <Route path=path!("/resume") view=ResumePage />
                            <Route path=path!("/contact") view=ContactPage />
                        </Routes>
                    </div>
                </main>

                // right sidebar
                <SidebarRight />
            </div>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col items-start justify-center min-h-[60vh] gap-6">
            <div class="text-6xl font-bold bg-gradient-to-r from-accent to-muted bg-clip-text text-transparent">
                "404 :: NOT_FOUND"
            </div>
            <p class="text-xl text-muted">"System error: Segment fault at requested path."</p>
            <a href="/" class="btn-primary">
                "Return to root"
            </a>
        </div>
    }
}
