use leptos::prelude::*;

#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    // we cycle through these themes
    let themes = ["dark", "light", "solarized", "high-contrast"];
    let (current_theme, set_current_theme) = signal::<String>("dark".to_string());

    // effect to initialize the theme state from what the <script> set
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let window = window();
            if let Some(doc) = window.document() {
                if let Some(el) = doc.document_element() {
                    let mut found = "dark".to_string();
                    if let Some(theme) = el.get_attribute("data-theme") {
                        found = theme;
                    } else if el.class_list().contains("dark") {
                        found = "dark".to_string();
                        let _ = el.set_attribute("data-theme", "dark");
                        let _ = el.class_list().remove_1("dark");
                    }
                    set_current_theme.set(found);
                }
            }
        }
    });

    let cycle_theme = move |_| {
        let current = current_theme.get();
        let current_idx = themes.iter().position(|&t| t == current).unwrap_or(0);
        let next_idx = (current_idx + 1) % themes.len();
        let next = themes[next_idx];
        set_current_theme.set(next.to_string());

        #[cfg(target_arch = "wasm32")]
        {
            let window = window();
            if let Some(doc) = window.document() {
                if let Some(el) = doc.document_element() {
                    let _ = el.set_attribute("data-theme", next);
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.set_item("theme", next);
                    }
                }
            }
        }
    };

    view! {
        <button
            class="flex items-center gap-2 p-2 text-muted hover:text-txt transition-colors w-full text-left font-mono text-sm"
            on:click=cycle_theme
            aria-label="Cycle theme"
            title="Cycle theme"
        >
            <svg class="w-4 h-4 shrink-0 svg-icon" style="width: 1em;height: 1em;vertical-align: middle;fill: currentColor;overflow: hidden;" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg">
                <path d="M924.8 337.6a449.344 449.344 0 1 0-828.288 348.8 449.344 449.344 0 0 0 828.288-348.8z m-448.768 544.64A368.768 368.768 0 0 1 248.96 775.04a371.968 371.968 0 0 1 0-525.952 369.152 369.152 0 0 1 226.944-107.264v740.48z m298.944-633.216c10.112 10.048 19.584 20.672 28.416 31.808l-255.36 255.36V457.088l217.344-217.344c3.264 3.008 6.464 6.144 9.6 9.344z m60.48 78.976c9.856 17.28 18.304 35.328 25.216 54.016l-312.704 312.576V615.424l287.488-287.424z m42.304 116.032c4.16 22.4 6.208 45.184 6.272 67.968l-0.064 5.12-336 335.872v-79.168l329.792-329.792z m-156.352-239.616l-173.44 173.44V298.624l123.136-123.072c17.536 8.32 34.304 17.92 50.304 28.864z m-107.904-50.432L547.968 219.52V141.696c22.208 2.176 44.16 6.272 65.536 12.288z m161.472 620.992a369.664 369.664 0 0 1-166.08 96.256l262.4-262.336a369.728 369.728 0 0 1-96.32 166.08z"  />
            </svg>
            <span class="hidden md:inline">"Theme: " {move || current_theme.get()}</span>
        </button>
    }
}
