use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Shriyans Arkal — About" />

        <div class="max-w-3xl">
            <header class="mb-12">
                <h1 class="text-3xl font-bold tracking-tight text-txt mb-2">"Shriyans Arkal"</h1>
                <div class="text-muted text-sm flex items-center gap-2">
                    <span>"Software Engineer and M.S. Computer Science student at the University of Florida."</span>
                </div>
            </header>

            <section class="mb-12">
                <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-4">"Overview"</h2>
                <div class="prose prose-invert max-w-none text-muted leading-relaxed space-y-4 text-sm sm:text-base">
                    <p>
                        "I build AI systems, agent frameworks, and the infrastructure behind them. Most of my work sits at the intersection of machine learning, full-stack development, and systems programming."
                    </p>
                    <p>
                        "Currently deep in the Rust ecosystem, exploring what agentic AI looks like when performance actually matters."
                    </p>
                </div>
            </section>

            <section>
                <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-4">"Links"</h2>
                <div class="flex flex-wrap gap-4">
                    <a href="https://github.com/s-arkal" target="_blank" rel="noopener noreferrer" class="btn-secondary text-sm px-4 py-2">
                        "GitHub ->"
                    </a>
                    <a href="https://www.linkedin.com/in/shriyansa/" target="_blank" rel="noopener noreferrer" class="btn-secondary text-sm px-4 py-2">
                        "LinkedIn ->"
                    </a>
                    <a href="/resume" class="btn-secondary text-sm px-4 py-2">
                        "Resume ->"
                    </a>
                </div>
            </section>
        </div>
    }
}
