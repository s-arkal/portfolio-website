use leptos::prelude::*;

#[component]
fn SkillBadge(text: &'static str, icon: &'static str) -> impl IntoView {
    view! {
        <span class="px-2 py-1 text-[10px] bg-panel border border-bdr text-txt whitespace-nowrap flex items-center gap-1.5 rounded-sm">
            <Show when=move || !icon.is_empty()>
                <i class=format!("{} text-xs", icon)></i>
            </Show>
            {text}
        </span>
    }
}

#[component]
fn SkillCategory(title: &'static str, skills: Vec<(&'static str, &'static str)>) -> impl IntoView {
    view! {
        <div class="mb-5">
            <div class="text-[9px] font-bold tracking-widest text-muted uppercase mb-2 border-b border-bdr pb-1">
                {title}
            </div>
            <div class="flex flex-wrap gap-1.5">
                {skills.into_iter().map(|(s, i)| view! { <SkillBadge text=s icon=i /> }).collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn SidebarRight() -> impl IntoView {
    let (commit_hash, set_commit_hash) = signal("-------".to_string());

    #[cfg(feature = "hydrate")]
    {
        use leptos::task::spawn_local;
        use gloo_net::http::Request;
        use serde::Deserialize;

        #[derive(Deserialize)]
        struct GithubCommit {
            sha: String,
        }

        spawn_local(async move {
            let url = "https://api.github.com/repos/s-arkal/portfolio-website/commits/main";
            if let Ok(resp) = Request::get(url).send().await {
                if let Ok(commit) = resp.json::<GithubCommit>().await {
                    set_commit_hash.set(commit.sha.chars().take(7).collect());
                } else {
                    set_commit_hash.set("api_err".to_string());
                }
            } else {
                set_commit_hash.set("offline".to_string());
            }
        });
    }

    view! {
        <aside class="flex flex-col w-full xl:w-64 max-w-none xl:max-w-[16rem] flex-shrink-0 border-t xl:border-t-0 xl:border-l border-bdr bg-panel h-auto xl:h-full p-6 xl:overflow-y-auto mt-8 xl:mt-0">
            
            <div class="mb-4">
                <div class="text-xs font-bold tracking-widest text-txt uppercase mb-6 flex items-center justify-between">
                    "Tech Stack"
                    <svg class="w-4 h-4 text-muted" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
                </div>
                
                <SkillCategory title="AI & Agents" skills=vec![
                    ("Pydantic AI", "devicon-python-plain text-pink-500"), ("CrewAI", ""), ("LangChain", ""), 
                    ("PyTorch", "devicon-pytorch-original colored"), ("RAG", ""), ("Fine-tuning", "")
                ] />
                <SkillCategory title="Languages" skills=vec![
                    ("Python", "devicon-python-plain colored"), ("Rust", "devicon-rust-plain"), ("TypeScript", "devicon-typescript-plain colored"), 
                    ("JavaScript", "devicon-javascript-plain colored"), ("Java", "devicon-java-plain colored"), ("C", "devicon-c-plain colored"), ("SQL", "devicon-azuresqldatabase-plain colored")
                ] />
                <SkillCategory title="Cloud & DevOps" skills=vec![
                    ("AWS", "devicon-amazonwebservices-plain-wordmark colored"), ("Azure", "devicon-azure-plain colored"), ("GCP", "devicon-googlecloud-plain colored"), 
                    ("Docker", "devicon-docker-plain colored"), ("CI/CD", "")
                ] />
                <SkillCategory title="Systems & Backend" skills=vec![
                    ("WebAssembly", "devicon-wasm-original colored"), ("Linux", "devicon-linux-plain"), ("FastAPI", "devicon-fastapi-plain colored")
                ] />
                <SkillCategory title="Tools" skills=vec![
                    ("Git", "devicon-git-plain colored"), ("VS Code", "devicon-vscode-plain colored"), 
                    ("Jupyter", "devicon-jupyter-plain colored"), ("Postman", "devicon-postman-plain colored")
                ] />
            </div>

            <div class="mt-auto pt-4 border-t border-bdr font-mono">
                <div class="flex items-center justify-between w-full">
                    <div class="flex items-center gap-2">
                        <span class="inline-block w-2 h-2 rounded-full bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.6)] animate-pulse"></span>
                        <span class="text-[10px] text-txt font-bold tracking-wider">
                            {format!("v{}", env!("CARGO_PKG_VERSION"))}
                        </span>
                    </div>
                    <span class="text-[10px] text-muted">
                        {move || commit_hash.get()}
                    </span>
                </div>
            </div>
            
        </aside>
    }
}
