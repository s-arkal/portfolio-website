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
                        "I build AI systems, and the infrastructure behind them. Currently, my work sits at the intersection of machine learning, full-stack development, and systems programming."
                    </p>
                    <p>
                        "As of now, I am deep in the "
                        <a href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer" class="text-accent hover:underline">"Rust"</a>
                        " ecosystem, exploring the intersection of agentic AI and high-performance systems engineering."
                    </p>
                </div>
            </section>

            <section>
                <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-4">"Links"</h2>
                <div class="flex flex-wrap gap-4">
                    <a href="https://github.com/s-arkal" target="_blank" rel="noopener noreferrer" aria-label="View my GitHub Profile" class="btn-secondary text-sm px-4 py-2 flex items-center gap-2 focus:outline-none focus:ring-2 focus:ring-accent">
                        <i class="devicon-github-original text-lg"></i>
                        "GitHub"
                    </a>
                    <a href="https://www.linkedin.com/in/shriyansa/" target="_blank" rel="noopener noreferrer" aria-label="View my LinkedIn Profile" class="btn-secondary text-sm px-4 py-2 flex items-center gap-2 focus:outline-none focus:ring-2 focus:ring-accent">
                        <i class="devicon-linkedin-plain text-lg"></i>
                        "LinkedIn"
                    </a>
                    <a href="/resume" aria-label="View Resume" class="btn-secondary text-sm px-4 py-2 flex items-center gap-2 focus:outline-none focus:ring-2 focus:ring-accent">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
                        "Resume"
                    </a>
                </div>
            </section>
        </div>
    }
}
