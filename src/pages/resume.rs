use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn ResumePage() -> impl IntoView {
    view! {
        <Title text="Resume — Shriyans Arkal" />

        <div class="max-w-3xl h-full flex flex-col">
            <header class="mb-8">
                <div class="flex items-start justify-between">
                    <div>
                        <h1 class="text-3xl font-bold tracking-tight text-txt mb-2">"Resume"</h1>
                        <p class="text-muted text-sm leading-relaxed">
                            "A detailed history of my academic and professional background."
                        </p>
                    </div>
                    <a href="/resume.pdf" target="_blank" class="btn-primary text-sm whitespace-nowrap">
                        "[ Download_PDF ]"
                    </a>
                </div>
            </header>

            <div class="flex-1 w-full bg-panel border-2 border-dashed border-bdr rounded-lg flex items-center justify-center min-h-[500px]">
                <div class="text-center p-6">
                    <svg class="w-12 h-12 text-muted mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <p class="text-muted font-mono text-sm">"Embed PDF viewer here"</p>
                    <p class="text-xs text-muted mt-2">"Currently waiting for resume.pdf to be placed in /public"</p>
                </div>
            </div>
        </div>
    }
}
