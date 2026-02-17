#![allow(non_snake_case)]

use dioxus::document::eval;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

use super::sections::{CTASection, ProductsSection, StatsSection, WorkflowSection};
use crate::jello::render::BodyRenderData;
use crate::jello::{create_physics, render, PhysicsState};

#[allow(clippy::cast_possible_truncation)]
fn coord_f32(v: f64) -> f32 {
    v as f32
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn dim_u32(v: f32) -> u32 {
    v.max(0.0) as u32
}

#[component]
pub fn Home() -> Element {
    use_effect(move || {
        let _ = eval("window.AppBridge && window.AppBridge.init();");
    });

    rsx! {
        HeroSection {}
        ProductsSection {}
        WorkflowSection {}
        StatsSection {}
        CTASection {}
    }
}

fn HeroSection() -> Element {
    let mut scene = use_signal::<Option<(PhysicsState, Vec<BodyRenderData>)>>(|| None);
    let mut canvas_el = use_signal::<Option<HtmlCanvasElement>>(|| None);
    let mut is_mobile = use_signal(|| true);

    use_effect(move || {
        if let Some(win) = window() {
            let width = win
                .inner_width()
                .map(|w| w.as_f64().unwrap_or(0.0))
                .unwrap_or(0.0);
            is_mobile.set(width < 768.0);
        }
    });

    let on_mouse_move = move |evt: MouseEvent| {
        let x = coord_f32(evt.client_coordinates().x);
        let y = coord_f32(evt.client_coordinates().y);
        scene.with_mut(|s| {
            if let Some((state, _)) = s {
                state.update_mouse(x, y);
            }
        });
    };

    use_effect(move || {
        let Some(el) = canvas_el.read().clone() else {
            return;
        };

        spawn(async move {
            loop {
                scene.with_mut(|s| {
                    if let Some((state, rd)) = s {
                        state.step(0.016);
                        if let Ok(Some(ctx)) = el.get_context("2d") {
                            if let Ok(ctx) = ctx.dyn_into::<CanvasRenderingContext2d>() {
                                render::draw(
                                    &ctx,
                                    state.bodies(),
                                    rd,
                                    state.width(),
                                    state.height(),
                                );
                            }
                        }
                    }
                });
                TimeoutFuture::new(16).await;
            }
        });
    });

    if !*is_mobile.read() {
        rsx! {
            div {
                style: "
                    width: 100%;
                    min-height: 90vh;
                    position: relative;
                    background: #08080c;
                    overflow: hidden;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                ",
                onmousemove: on_mouse_move,

                canvas {
                    id: "jello-canvas",
                    style: "
                        position: absolute;
                        top: 0;
                        left: 0;
                        width: 100%;
                        height: 100%;
                        cursor: crosshair;
                        z-index: 1;
                    ",

                    onmounted: move |_evt| {
                        spawn(async move {
                            if let Some(win) = window() {
                                let doc = win.document();
                                if let Some(doc) = doc {
                                    let fonts = doc.fonts();
                                    let promise = fonts.ready();
                                    let _ = JsFuture::from(promise).await;
                                }
                            }

                            TimeoutFuture::new(100).await;

                            let Some(win) = window() else { return };
                            let Some(doc) = win.document() else { return };
                            let Some(canvas) = doc.get_element_by_id("jello-canvas") else {
                                web_sys::console::log_1(&"[JELLO] Failed to find canvas".into());
                                return;
                            };

                            let Ok(el) = canvas.dyn_into::<HtmlCanvasElement>() else {
                                web_sys::console::log_1(&"[JELLO] Not a canvas".into());
                                return;
                            };

                            canvas_el.set(Some(el.clone()));

                            let Some(parent) = el.parent_element() else { return };
                            let rect = parent.get_bounding_client_rect();
                            let w = coord_f32(rect.width()).max(100.0);
                            let h = coord_f32(rect.height()).max(100.0);

                            el.set_width(dim_u32(w));
                            el.set_height(dim_u32(h));

                            match create_physics(&win, "WE BUILD TOOLS", w, h) {
                                Some(p) => {
                                    web_sys::console::log_1(&format!("[JELLO] {} bodies", p.0.bodies().len()).into());
                                    scene.set(Some(p));
                                }
                                None => {
                                    web_sys::console::log_1(&"[JELLO] Init failed (Stubbed or Error)".into());
                                }
                            }
                        });
                    }
                }

                div {
                    style: "
                        position: relative;
                        z-index: 10;
                        text-align: center;
                        padding: 2rem;
                        max-width: 800px;
                        pointer-events: none;
                    ",

                    div { style: "height: 20vh;" }

                    p {
                        style: "
                            font-family: var(--font-mono);
                            font-size: 1.1rem;
                            color: var(--text-soft);
                            margin-bottom: 2rem;
                            line-height: 1.6;
                        ",
                        "Tools that watch your architecture."
                    }

                    div {
                        style: "display: flex; gap: 1rem; justify-content: center; pointer-events: auto;",

                        a {
                            class: "btn btn--primary btn--large",
                            href: "https://github.com/semmaplabs",
                            target: "_blank",
                            "View on GitHub"
                        }
                        a {
                            class: "btn btn--secondary btn--large",
                            href: "#products",
                            "Learn More"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "hero",
                style: "
                    min-height: 90vh;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    text-align: center;
                    animation: fadeInUp 0.6s ease both;
                ",

                h1 {
                    class: "hero__title",
                    style: "font-size: clamp(2.5rem, 8vw, 4rem); margin-bottom: 1.5rem;",
                    "WE BUILD TOOLS"
                }

                p {
                    style: "
                        font-family: var(--font-mono);
                        font-size: 1rem;
                        color: var(--text-soft);
                        margin-bottom: 2.5rem;
                        max-width: 90%;
                    ",
                    "Tools that watch your architecture."
                }

                div {
                    style: "display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap;",

                    a {
                        class: "btn btn--primary btn--large",
                        href: "https://github.com/semmaplabs",
                        target: "_blank",
                        "View on GitHub"
                    }
                    a {
                        class: "btn btn--secondary btn--large",
                        href: "#products",
                        "Learn More"
                    }
                }
            }
        }
    }
}
