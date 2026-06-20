use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <Title text="Contact — Shriyans Arkal" />

        <div class="max-w-3xl">
            <header class="mb-12">
                <h1 class="text-3xl font-bold tracking-tight text-txt mb-2">"Contact"</h1>
                <p class="text-muted text-sm leading-relaxed">
                    "Feel free to reach out!"
                </p>
            </header>

            <section>
                    <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-6 flex items-center gap-4">
                        "Contact"
                        <div class="h-px bg-bdr flex-1"></div>
                    </h2>
                    <ul class="space-y-4">
                        <li>
                            <div class="text-xs font-mono text-muted mb-1">"Email"</div>
                            <a href="mailto:shriyans.arkal07@gmail.com" aria-label="Send an email to shriyans.arkal07@gmail.com" class="text-sm font-medium text-txt hover:text-accent transition-colors focus:outline-none focus:ring-2 focus:ring-accent inline-block">
                                "shriyans.arkal07@gmail.com"
                            </a>
                        </li>
                        <li>
                            <div class="text-xs font-mono text-muted mb-1">"GitHub"</div>
                            <a href="https://github.com/s-arkal" target="_blank" rel="noopener noreferrer" aria-label="View my GitHub Profile" class="text-sm font-medium text-txt hover:text-accent transition-colors focus:outline-none focus:ring-2 focus:ring-accent inline-block">
                                "github.com/s-arkal"
                            </a>
                        </li>
                        <li>
                            <div class="text-xs font-mono text-muted mb-1">"LinkedIn"</div>
                            <a href="https://www.linkedin.com/in/shriyansa/" target="_blank" rel="noopener noreferrer" aria-label="View my LinkedIn Profile" class="text-sm font-medium text-txt hover:text-accent transition-colors focus:outline-none focus:ring-2 focus:ring-accent inline-block">
                                "linkedin.com/in/shriyansa"
                            </a>
                        </li>
                    </ul>
            </section>
        </div>
    }
}
