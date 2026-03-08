use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn ContactPage() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (submitted, set_submitted) = signal(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        // todo: impl form
        set_submitted.set(true);
    };

    view! {
        <Title text="Contact — Shriyans Arkal" />

        <div class="max-w-3xl">
            <header class="mb-12">
                <h1 class="text-3xl font-bold tracking-tight text-txt mb-2">"Contact"</h1>
                <p class="text-muted text-sm leading-relaxed">
                    "Feel free to reach out!"
                </p>
            </header>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-12">
                // direct links
                <div>
                    <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-6 flex items-center gap-4">
                        "Links"
                        <div class="h-px bg-bdr flex-1"></div>
                    </h2>
                    <ul class="space-y-4">
                        <li>
                            <div class="text-xs font-mono text-muted mb-1">"Email"</div>
                            <a href="mailto:shriyans.arkal07@gmail.com" class="text-sm font-medium text-txt hover:text-accent transition-colors">
                                "shriyans.arkal07@gmail.com"
                            </a>
                        </li>
                        <li>
                            <div class="text-xs font-mono text-muted mb-1">"GitHub"</div>
                            <a href="https://github.com/s-arkal" target="_blank" rel="noopener noreferrer" class="text-sm font-medium text-txt hover:text-accent transition-colors">
                                "github.com/s-arkal"
                            </a>
                        </li>
                        <li>
                            <div class="text-xs font-mono text-muted mb-1">"LinkedIn"</div>
                            <a href="https://www.linkedin.com/in/shriyansa/" target="_blank" rel="noopener noreferrer" class="text-sm font-medium text-txt hover:text-accent transition-colors">
                                "linkedin.com/in/shriyansa"
                            </a>
                        </li>
                    </ul>
                </div>

                // contact form
                <div>
                    <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-6 flex items-center gap-4">
                        "Message"
                        <div class="h-px bg-bdr flex-1"></div>
                    </h2>
                    
                    <Show
                        when=move || submitted.get()
                        fallback=move || view! {
                            <form on:submit=on_submit class="space-y-4">
                                <div>
                                    <label for="name" class="block text-xs font-mono text-muted mb-1">"Name"</label>
                                    <input
                                        id="name"
                                        type="text"
                                        class="input-field text-sm"
                                        required=true
                                        on:input=move |ev| set_name.set(event_target_value(&ev))
                                        prop:value=move || name.get()
                                    />
                                </div>
                                <div>
                                    <label for="email" class="block text-xs font-mono text-muted mb-1">"Email"</label>
                                    <input
                                        id="email"
                                        type="email"
                                        class="input-field text-sm"
                                        required=true
                                        on:input=move |ev| set_email.set(event_target_value(&ev))
                                        prop:value=move || email.get()
                                    />
                                </div>
                                <div>
                                    <label for="message" class="block text-xs font-mono text-muted mb-1">"Payload"</label>
                                    <textarea
                                        id="message"
                                        rows="4"
                                        class="input-field resize-none text-sm"
                                        required=true
                                        on:input=move |ev| set_message.set(event_target_value(&ev))
                                        prop:value=move || message.get()
                                    />
                                </div>
                                <div class="pt-2 flex justify-end">
                                    <button type="submit" class="btn-primary text-sm">
                                        "[ Submit_Query ]"
                                    </button>
                                </div>
                            </form>
                        }
                    >
                        <div class="card bg-hover border-accent/20 p-6 flex flex-col items-center justify-center text-center">
                            <svg class="w-8 h-8 text-accent mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <h3 class="font-bold text-txt mb-2">"Transmission Successful"</h3>
                            <p class="text-sm text-muted">"Message logged to the queue. I'll get back to you shortly."</p>
                        </div>
                    </Show>
                </div>
            </div>
        </div>
    }
}
