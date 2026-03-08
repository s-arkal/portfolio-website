use leptos::prelude::*;

#[component]
pub fn SidebarRight() -> impl IntoView {
    view! {
        <aside class="hidden lg:block w-64 flex-shrink-0 border-l border-bdr bg-panel h-full p-6 overflow-y-auto">
            <div class="pt-6">
                <div class="text-xs font-bold tracking-widest text-muted uppercase mb-4">
                    "idk what to put here"
                </div>
                <div class="flex items-center gap-2 text-sm text-txt">
                    <span class="inline-block w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
                    <span>"All systems operational"</span>
                </div>
                <div class="mt-2 text-xs text-muted font-mono">
                    "????"
                </div>
            </div>
        </aside>
    }
}
