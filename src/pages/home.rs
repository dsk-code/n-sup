use leptos::prelude::*;
use crate::components::{Header, HeroSection, FeaturesSection, BenefitsSection, CtaSection, Footer};
use crate::utils::{setup_scroll_animations, setup_all_3d_effects, setup_all_scroll_handlers, setup_smooth_scrolling};
use wasm_bindgen::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    // Set up all animations and interactions after component mounts
    Effect::new(move |_| {
        let setup_closure = Closure::wrap(Box::new(move || {
            setup_scroll_animations();
            setup_all_3d_effects();
            setup_all_scroll_handlers();
            setup_smooth_scrolling();
        }) as Box<dyn FnMut()>);
        
        let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
            setup_closure.as_ref().unchecked_ref(),
            100
        );
        
        setup_closure.forget();
    });

    view! {
        <div class="subtle-grid"></div>
        <div class="ambient-bg"></div>
        <Header />
        <HeroSection />
        <FeaturesSection />
        <BenefitsSection />
        <CtaSection />
        <Footer />
    }
}

