use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn BackgroundPage() -> impl IntoView {
    view! {
        <Title text="Background — Shriyans Arkal" />

        <div class="max-w-3xl">
            <header class="mb-12">
                <h1 class="text-3xl font-bold tracking-tight text-txt mb-2">"Background"</h1>
                <p class="text-muted text-sm leading-relaxed">
                    "My professional experience and academic history so far."
                </p>
            </header>

            <div class="space-y-16">
                <section>
                    <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-6 flex items-center gap-4">
                        "Experience"
                        <div class="h-px bg-bdr flex-1"></div>
                    </h2>
                    <div class="space-y-8 relative before:absolute before:inset-0 before:ml-2 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-bdr before:to-transparent">
                        <ExperienceEntry
                            role="AI Agent Developer Intern (Enterprise CTO Office)"
                            company="Unisys Corporation | Bengaluru, India"
                            period="Sep 2024 – Jul 2025"
                            description="Architected multi-agent AI pipelines to automate workflows using CrewAI and Azure AI Tools."
                        />
                    </div>
                </section>

                <section>
                    <h2 class="text-xs font-bold tracking-widest text-muted uppercase mb-6 flex items-center gap-4">
                        "Education"
                        <div class="h-px bg-bdr flex-1"></div>
                    </h2>
                    <div class="space-y-8 relative before:absolute before:inset-0 before:ml-2 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-bdr before:to-transparent">
                        <EducationEntry
                            degree="M.S. in Computer Science"
                            school="University of Florida | Gainesville, FL, USA"
                            period="Aug 2025 – May 2027"
                        />
                        <EducationEntry
                            degree="B.Tech. in Computer Science (AI & ML)"
                            school="Dayananda Sagar University | Bengaluru, Karnataka, India"
                            period="Oct 2021 – Jul 2025"
                        />
                    </div>
                </section>
            </div>
        </div>
    }
}

#[component]
fn ExperienceEntry(
    role: &'static str,
    company: &'static str,
    period: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group">
            <div class="flex items-center justify-center w-5 h-5 rounded-full border-2 border-bg bg-bdr group-hover:bg-accent group-hover:border-accent/30 transition-colors shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 absolute left-0 md:left-1/2 -mb-1 z-10"></div>
            
            <div class="w-[calc(100%-2rem)] md:w-[calc(50%-1.5rem)] ml-8 md:ml-0 card group-hover:border-muted transition-colors p-5">
                <div class="flex flex-col xl:flex-row xl:items-baseline justify-between gap-1 mb-2">
                    <h3 class="font-bold text-txt">{role}</h3>
                    <span class="text-xs font-mono text-accent whitespace-nowrap">{period}</span>
                </div>
                <div class="text-sm font-medium text-muted mb-4">{company}</div>
                <p class="text-sm text-muted leading-relaxed">
                    {description}
                </p>
            </div>
        </div>
    }
}

#[component]
fn EducationEntry(
    degree: &'static str,
    school: &'static str,
    period: &'static str,
) -> impl IntoView {
    view! {
        <div class="relative flex items-center justify-between md:justify-normal md:odd:flex-row-reverse group">
            <div class="flex items-center justify-center w-5 h-5 rounded-full border-2 border-bg bg-bdr group-hover:bg-accent group-hover:border-accent/30 transition-colors shadow shrink-0 md:order-1 md:group-odd:-translate-x-1/2 md:group-even:translate-x-1/2 absolute left-0 md:left-1/2 -mb-1 z-10"></div>
            
            <div class="w-[calc(100%-2rem)] md:w-[calc(50%-1.5rem)] ml-8 md:ml-0 card group-hover:border-muted transition-colors p-5">
                <div class="flex flex-col xl:flex-row xl:items-baseline justify-between gap-1 mb-2">
                    <h3 class="font-bold text-txt">{degree}</h3>
                    <span class="text-xs font-mono text-accent whitespace-nowrap">{period}</span>
                </div>
                <div class="text-sm font-medium text-muted">{school}</div>
            </div>
        </div>
    }
}
