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
                // ai systems category
                <section>
                    <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-6 flex items-center gap-4">
                        "AI Systems, Infrastructure & Full-stack"
                        <div class="h-px bg-bdr flex-1"></div>
                    </h2>
                    <div class="grid grid-cols-1 gap-6">
                        <ProjectCard
                            title="Cogito: Autonomous AI Academic Research Workspace"
                            description="An autonomous research platform capable of multi-step reasoning and autonomous literature review. Allows researchers to dynamically query dense sets of documents, autonomously ingest external citations into a unified knowledge graph, and author documents with real-time AI copilot assistance."
                            labels=vec!["[Pydantic AI]", "[ChromaDB]", "[WebAssembly]", "[SQLite]", "[FastAPI]", "[Next.js]"]
                            status="WIP"
                            github_link="https://github.com/shriyan-s/cogito"
                        />
                        <ProjectCard
                            title="Relay"
                            description="An agentic AI framework in Rust, because why not?"
                            labels=vec!["[Rust]", "[AI]", "[Agents]"]
                            status="WIP"
                            github_link="https://github.com/shriyan-s/relay"
                        />
                        <ProjectCard
                            title="rust-leptos-portfolio"
                            description="This very portfolio site. Built entirely in Rust using Leptos and WebAssembly, featuring a custom 4-theme system, responsive 3-column layout, and server-side rendering."
                            labels=vec!["[Rust]", "[Leptos]", "[WASM]", "[Tailwind]"]
                            status="Active"
                            github_link="https://github.com/s-arkal/rust-leptos-portfolio"
                        />
                    </div>
                </section>

                // academic & coursework category
                <section>
                    <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-6 flex items-center gap-4">
                        "Systems & Data Structures"
                        <div class="h-px bg-bdr flex-1"></div>
                    </h2>
                    <div class="grid grid-cols-1 gap-6">
                        <ProjectCardDetailed
                            title="Gator Air Traffic Scheduler"
                            bullets=vec![
                                "Built a high-performance Pythonair traffic control simulation system using advanced data structures (Max Pairing Heap, Binary Min Heap).",
                                "Engineered an efficient, non-preemptive logic to manage flight scheduling, reprioritization, and cancellations."
                            ]
                            labels=vec!["[Python]", "[Data Structures]", "[Algorithms]"]
                            status="Academic"
                            github_link="https://github.com/shriyan-s"
                        />
                        <ProjectCardDetailed
                            title="OOPascal"
                            bullets=vec![
                                "Developed a Java-based interpreter extending predefined Pascal grammar with object-oriented features (classes, constructors, encapsulation, inheritance).",
                                "Built a custom Lexer, Parser, and AST evaluator using ANTLR4, validating functionality across comprehensive OOP test suites."
                            ]
                            labels=vec!["[Java]", "[ANTLR4]", "[Compilers]", "[Pascal]"]
                            status="Academic"
                            github_link="https://github.com/shriyan-s"
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
    status: &'static str,
    github_link: &'static str,
) -> impl IntoView {
    let status_color = match status {
        "Active" => "text-green-500",
        "WIP" => "text-blue-500",
        "Academic" => "text-yellow-500",
        "Archived" => "text-red-500",
        _ => "text-muted",
    };

    view! {
        <a href=github_link target="_blank" rel="noopener noreferrer" class="card group flex flex-col h-full hover:bg-hover">
            <div class="flex items-start justify-between mb-4">
                <h3 class="text-base font-bold text-txt group-hover:text-accent transition-colors flex items-center gap-2">
                    <svg class="w-4 h-4 text-muted" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                    </svg>
                    {title}
                </h3>
            </div>
            
            <p class="text-muted text-sm leading-relaxed flex-1 mb-6">
                {description}
            </p>
            
            <div class="flex items-center justify-between mt-auto pt-4 border-t border-bdr">
                <div class="flex flex-wrap gap-2">
                    {labels.into_iter().map(|label| {
                        view! {
                            <span class="text-xs font-mono text-muted">
                                {label}
                            </span>
                        }
                    }).collect_view()}
                </div>
                <div class=format!("text-xs font-mono {} flex items-center gap-1.5 shrink-0 ml-4", status_color)>
                    <span class="w-1.5 h-1.5 rounded-full bg-current"></span>
                    {status}
                </div>
            </div>
        </a>
    }
}

#[component]
fn ProjectCardDetailed(
    title: &'static str,
    bullets: Vec<&'static str>,
    labels: Vec<&'static str>,
    status: &'static str,
    github_link: &'static str,
) -> impl IntoView {
    let status_color = match status {
        "Active" => "text-green-500",
        "WIP" => "text-blue-500",
        "Academic" => "text-yellow-500",
        "Archived" => "text-red-500",
        _ => "text-muted",
    };

    view! {
        <a href=github_link target="_blank" rel="noopener noreferrer" class="card group flex flex-col h-full hover:bg-hover">
            <div class="flex items-start justify-between mb-4">
                <h3 class="text-base font-bold text-txt group-hover:text-accent transition-colors flex items-center gap-2">
                    <svg class="w-4 h-4 text-muted" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                    </svg>
                    {title}
                </h3>
            </div>
            
            <ul class="text-muted text-sm leading-relaxed flex-1 mb-6 list-disc pl-4 space-y-2">
                {bullets.into_iter().map(|bullet| {
                    view! { <li>{bullet}</li> }
                }).collect_view()}
            </ul>
            
            <div class="flex items-center justify-between mt-auto pt-4 border-t border-bdr">
                <div class="flex flex-wrap gap-2">
                    {labels.into_iter().map(|label| {
                        view! {
                            <span class="text-xs font-mono text-muted">
                                {label}
                            </span>
                        }
                    }).collect_view()}
                </div>
                <div class=format!("text-xs font-mono {} flex items-center gap-1.5 shrink-0 ml-4", status_color)>
                    <span class="w-1.5 h-1.5 rounded-full bg-current"></span>
                    {status}
                </div>
            </div>
        </a>
    }
}
