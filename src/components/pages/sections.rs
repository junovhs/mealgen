#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn ProductsSection() -> Element {
    rsx! {
        section { id: "products", class: "products-section",
            div { class: "container",
                div { class: "section-header",
                    span { class: "section-label", "Products" }
                    h2 { class: "section-title", "What We Build" }
                    p { class: "section-desc",
                        "Developer tools focused on code quality, structural governance, \
                         and AI workflow optimization."
                    }
                }

                div { class: "product-grid",
                    ProductCard {
                        icon: "T",
                        name: "Talos",
                        role: "Architectural Linter",
                        desc: "A governance engine that enforces structural laws in your codebase. \
                               Catches complexity violations, coupling issues, and architectural \
                               drift before they become technical debt.",
                        tags: vec!["Complexity Analysis", "Coupling Detection", "CI/CD Ready", "Rust/TS/Python"],
                        link: "https://github.com/semmaplabs/talos",
                        link_text: "Learn More"
                    }

                    ProductCard {
                        icon: "S",
                        name: "SEMMAP",
                        role: "Semantic Mapping",
                        desc: "Generates semantic maps of codebases for AI consumption. \
                               Enables progressive disclosure so LLMs can navigate large \
                               repositories without token explosions.",
                        tags: vec!["AI Context", "Token Efficiency", "Progressive Disclosure", "Multi-Language"],
                        link: "https://github.com/semmaplabs/semmap",
                        link_text: "Learn More"
                    }

                    ProductCard {
                        icon: "D",
                        name: "DirAnalyze",
                        role: "File Analysis Tool",
                        desc: "In-browser tool for analyzing and combining directory contents. \
                               Perfect for creating context files for AI interactions. \
                               No upload, everything runs locally.",
                        tags: vec!["Browser-Based", "WASM Powered", "Privacy First", "Free Tool"],
                        link: "https://diranalyze.semmap.dev",
                        link_text: "Try Now"
                    }
                }

                div { style: "text-align: center;",
                    a { class: "btn btn--secondary",
                        href: "https://github.com/semmaplabs",
                        target: "_blank",
                        "View All Projects →"
                    }
                }
            }
        }
    }
}

#[component]
fn ProductCard(
    icon: String,
    name: String,
    role: String,
    desc: String,
    tags: Vec<&'static str>,
    link: String,
    link_text: String,
) -> Element {
    rsx! {
        div { class: "product-card",
            div { class: "product-card__icon", "{icon}" }
            h3 { class: "product-card__name", "{name}" }
            span { class: "product-card__role", "{role}" }
            p { class: "product-card__desc", "{desc}" }
            div { class: "product-card__features",
                for tag in tags {
                    span { class: "product-tag", "{tag}" }
                }
            }
            a { class: "product-card__cta", href: "{link}", target: "_blank", "{link_text}" }
        }
    }
}

pub fn WorkflowSection() -> Element {
    rsx! {
        section { class: "workflow-section",
            div { class: "container",
                div { class: "workflow-card",
                    div { class: "section-header", style: "margin-bottom: 0;",
                        span { class: "section-label", "Workflow" }
                        h2 { class: "section-title", "Context + Constraint" }
                    }

                    div { class: "workflow-steps",
                        div { class: "workflow-step",
                            div { class: "workflow-step__icon", "🗺" }
                            h4 { class: "workflow-step__name", "SEMMAP" }
                            p { class: "workflow-step__desc", "Provides the map" }
                        }

                        div { class: "workflow-arrow", "→" }

                        div { class: "workflow-step",
                            div { class: "workflow-step__icon", "🤖" }
                            h4 { class: "workflow-step__name", "LLM" }
                            p { class: "workflow-step__desc", "Writes the code" }
                        }

                        div { class: "workflow-arrow", "→" }

                        div { class: "workflow-step",
                            div { class: "workflow-step__icon", "🛡" }
                            h4 { class: "workflow-step__name", "TALOS" }
                            p { class: "workflow-step__desc", "Enforces the rules" }
                        }
                    }

                    p { style: "text-align: center; color: var(--text-soft); margin-top: var(--space-2xl); max-width: 600px; margin-left: auto; margin-right: auto;",
                        "A complete pipeline for reliable AI-assisted software development. \
                         SEMMAP gives the AI context, and TALOS ensures the output meets \
                         your architectural standards."
                    }
                }
            }
        }
    }
}

pub fn StatsSection() -> Element {
    rsx! {
        section { class: "stats-section",
            div { class: "container",
                div { class: "stats-grid",
                    div { class: "stat-item",
                        div { class: "stat-value", "4+" }
                        div { class: "stat-label", "Languages Supported" }
                    }
                    div { class: "stat-item",
                        div { class: "stat-value", "15" }
                        div { class: "stat-label", "Governance Rules" }
                    }
                    div { class: "stat-item",
                        div { class: "stat-value", "∞" }
                        div { class: "stat-label", "Token Savings" }
                    }
                    div { class: "stat-item",
                        div { class: "stat-value", "100%" }
                        div { class: "stat-label", "Open Source" }
                    }
                }
            }
        }
    }
}

pub fn CTASection() -> Element {
    rsx! {
        section { class: "cta-section",
            div { class: "cta-content",
                h2 { class: "cta-section__title", "Ready to Enforce the Law?" }
                p { class: "cta-section__desc",
                    "Stop AI slop before it hits your codebase. Start using TALOS and SEMMAP \
                     today to bring structural governance to your development workflow."
                }
                div { class: "hero__actions", style: "justify-content: center;",
                    a { class: "btn btn--primary btn--large",
                        href: "https://github.com/semmaplabs",
                        target: "_blank",
                        "Get Started on GitHub"
                    }
                    a { class: "btn btn--secondary btn--large",
                        href: "mailto:junovhs@gmail.com",
                        "Contact Us"
                    }
                }
            }
        }
    }
}
