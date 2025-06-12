use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let (is_scrolled, set_is_scrolled) = signal(false);

    Effect::new(move |_| {
        let window = window();
        let window_for_listener = window.clone();
        let closure = Closure::wrap(Box::new(move || {
            if let Ok(scroll_y) = window.scroll_y() {
                set_is_scrolled.set(scroll_y > 80.0);
            }
        }) as Box<dyn Fn()>);

        let _ = window_for_listener.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        closure.forget();
    });

    view! {
        <header class=move || { if is_scrolled.get() { "header header-scrolled" } else { "header" } }>
            <div class="header-content">
                <a href="#" class="logo">
                    "NSup"
                </a>
                <nav class="nav">
                    <a href="#features">"機能"</a>
                    <a href="#benefits">"メリット"</a>
                    <a href="#cta" class="header-cta">
                        "無料で始める"
                    </a>
                </nav>
            </div>
        </header>
    }
}