use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <Title text="Projects — Shriyans Arkal" />

        <div class="max-w-3xl">
            <header class="mb-12">
                <h1 class="text-3xl font-bold tracking-tight text-txt mb-2">"Projects"</h1>
                <p class="text-muted text-sm leading-relaxed">
                    "A selection of my projects."
                </p>
            </header>

            <div class="space-y-16">
                // ai and machine learning category
                <section>
                    <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-6 flex items-center gap-4">
                        "AI & Machine Learning"
                        <div class="h-px bg-bdr flex-1"></div>
                    </h2>
                    <div class="grid grid-cols-1 gap-6">
                        <ProjectCard
                            title="Relay"
                            description="An async-first Rust agent framework with a ReAct runtime, provider adapters, typed tools, approval gates, durable runs, memory/RAG, workflows, tracing, evals, and Python bindings."
                            labels=vec!["[Rust]", "[Tokio]", "[Agents]", "[RAG]", "[PyO3]"]
                            github_link="https://github.com/s-arkal/Relay"
                        />
                        <ProjectCard
                            title="AddGPT"
                            description="A 9.8M-parameter decoder-only Transformer implemented and trained from scratch for integer addition, with synthetic curriculum data, reversed targets, checkpointing, and per-digit exact-match evaluation."
                            labels=vec!["[Python]", "[PyTorch]", "[Transformer]", "[Training]", "[Colab]"]
                            github_link="https://github.com/s-arkal/AddGPT"
                        />
                    </div>
                </section>

                // full stack category
                <section>
                    <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-6 flex items-center gap-4">
                        "Full Stack"
                        <div class="h-px bg-bdr flex-1"></div>
                    </h2>
                    <div class="grid grid-cols-1 gap-6">
                        <ProjectCard
                            title="Cogito"
                            description="A full-stack academic research workspace with an AI orchestrator, web and paper discovery, PDF RAG, versioned notes, and a LaTeX manuscript editor with compile and preview support."
                            labels=vec!["[Pydantic AI]", "[Next.js]", "[FastAPI]", "[RAG]", "[LaTeX]"]
                            github_link="https://github.com/s-arkal/Cogito"
                        />
                        <ProjectCard
                            title="Respawn67"
                            description="A team-built gaming community platform with authentication, catalog browsing, backlogs and custom lists, ratings and reviews, guides, articles, favorites, and public user profiles."
                            labels=vec!["[React Router]", "[TypeScript]", "[Go]", "[Gin]", "[GORM]", "[SQLite]"]
                            github_link="https://github.com/bobby-likes-f1/respawn67"
                        />
                        <ProjectCard
                            title="Rust + Leptos Portfolio"
                            description="This portfolio site: a Leptos application with server-side rendering in a Cloudflare Worker, browser-side WebAssembly hydration, responsive theming, and push-triggered Pages deployments."
                            labels=vec!["[Rust]", "[Leptos]", "[WebAssembly]", "[Tailwind]", "[Cloudflare Pages]"]
                            github_link="https://github.com/s-arkal/portfolio-website"
                        />
                    </div>
                </section>

                // systems category
                <section>
                    <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-6 flex items-center gap-4">
                        "Systems"
                        <div class="h-px bg-bdr flex-1"></div>
                    </h2>
                    <div class="grid grid-cols-1 gap-6">
                        <ProjectCard
                            title="Gator Air Traffic Scheduler"
                            description="An event-driven air traffic scheduler built on custom binary min-heaps and a two-pass max pairing heap, supporting non-preemptive runway assignment, reprioritization, cancellation, and rescheduling."
                            labels=vec!["[Python]", "[Pairing Heap]", "[Binary Heap]", "[Scheduling]"]
                            github_link="https://github.com/s-arkal/gator-air-traffic-scheduler"
                        />
                        <ProjectCard
                            title="Object-Oriented Pascal Compiler"
                            description="An ANTLR4 front end with a Java interpreter and LLVM IR backend for a Pascal-like language, lowering classes, inheritance, procedures, functions, arithmetic, and control flow to native executables and WebAssembly."
                            labels=vec!["[Java]", "[ANTLR4]", "[LLVM IR]", "[Compiler]", "[WebAssembly]"]
                            github_link="https://github.com/kthomasuf/COP5556-Project/tree/llvm"
                        />
                    </div>
                </section>
            </div>
        </div>
    }
}

#[component]
fn ProjectCard(
    title: &'static str,
    description: &'static str,
    labels: Vec<&'static str>,
    github_link: &'static str,
) -> impl IntoView {
    view! {
        <a href=github_link aria-label=move || format!("View {} on GitHub", title) target="_blank" rel="noopener noreferrer" class="card group flex flex-col h-full hover:bg-hover focus:outline-none focus:ring-2 focus:ring-accent">
            <div class="flex items-start justify-between mb-4">
                <h3 class="text-base font-bold text-txt group-hover:text-accent transition-colors flex items-center gap-2">
                    <svg aria-hidden="true" class="w-4 h-4 text-muted" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                    </svg>
                    {title}
                </h3>
            </div>
            
            <p class="text-muted text-sm leading-relaxed flex-1 mb-6">
                {description}
            </p>
            
            <div class="flex flex-wrap gap-2 mt-auto pt-4 border-t border-bdr">
                {labels.into_iter().map(|label| {
                    view! {
                        <span class="text-xs font-mono text-muted">
                            {label}
                        </span>
                    }
                }).collect_view()}
            </div>
        </a>
    }
}
