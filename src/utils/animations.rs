use web_sys::*;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Card3DConfig {
    pub hover_lift: f64,
    pub rotation_factor: f64,
    pub scale_factor: f64,
    pub transform_template: &'static str,
}

impl Card3DConfig {
    pub fn feature_card() -> Self {
        Self {
            hover_lift: -8.0,
            rotation_factor: 20.0,
            scale_factor: 1.01,
            transform_template: "translateY({}px) rotateX({}deg) rotateY({}deg) scale({})",
        }
    }

    pub fn roi_card() -> Self {
        Self {
            hover_lift: -5.0,
            rotation_factor: 16.0,
            scale_factor: 1.01,
            transform_template: "translateY({}px) translateZ(15px) rotateX({}deg) rotateY({}deg) scale({})",
        }
    }

    pub fn cta_card() -> Self {
        Self {
            hover_lift: -5.0,
            rotation_factor: 30.0,
            scale_factor: 1.005,
            transform_template: "translateY({}px) rotateX({}deg) rotateY({}deg) scale({})",
        }
    }
}

pub fn setup_card_3d_effect(element: HtmlElement, config: Card3DConfig) {
    let element_clone = element.clone();
    let config_clone = config.clone();
    
    let mousemove_closure = Closure::wrap(Box::new(move |e: MouseEvent| {
        let rect = element_clone.get_bounding_client_rect();
        let x = e.client_x() as f64 - rect.left();
        let y = e.client_y() as f64 - rect.top();
        
        let center_x = rect.width() / 2.0;
        let center_y = rect.height() / 2.0;
        
        let rotate_x = (y - center_y) / config_clone.rotation_factor;
        let rotate_y = (center_x - x) / config_clone.rotation_factor;
        
        let transform = match config_clone.transform_template {
            "translateY({}px) rotateX({}deg) rotateY({}deg) scale({})" => format!(
                "translateY({}px) rotateX({}deg) rotateY({}deg) scale({})",
                config_clone.hover_lift, rotate_x, rotate_y, config_clone.scale_factor
            ),
            "translateY({}px) translateZ(15px) rotateX({}deg) rotateY({}deg) scale({})" => format!(
                "translateY({}px) translateZ(15px) rotateX({}deg) rotateY({}deg) scale({})",
                config_clone.hover_lift, rotate_x, rotate_y, config_clone.scale_factor
            ),
            _ => format!(
                "translateY({}px) rotateX({}deg) rotateY({}deg) scale({})",
                config_clone.hover_lift, rotate_x, rotate_y, config_clone.scale_factor
            ),
        };
        
        let _ = element_clone.style().set_property("transform", &transform);
    }) as Box<dyn FnMut(MouseEvent)>);
    
    let element_clone2 = element.clone();
    let mouseleave_closure = Closure::wrap(Box::new(move |_: MouseEvent| {
        let _ = element_clone2.style().set_property("transform", "");
    }) as Box<dyn FnMut(MouseEvent)>);
    
    let _ = element.add_event_listener_with_callback("mousemove", mousemove_closure.as_ref().unchecked_ref());
    let _ = element.add_event_listener_with_callback("mouseleave", mouseleave_closure.as_ref().unchecked_ref());
    
    mousemove_closure.forget();
    mouseleave_closure.forget();
}

pub fn setup_scroll_animations() {
    let window = match window() {
        Some(w) => w,
        None => {
            web_sys::console::error_1(&"Failed to get window object".into());
            return;
        }
    };
    
    let document = match window.document() {
        Some(d) => d,
        None => {
            web_sys::console::error_1(&"Failed to get document object".into());
            return;
        }
    };
    
    let observer_callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
        for entry_val in entries.iter() {
            let entry: IntersectionObserverEntry = entry_val.unchecked_into();
            if entry.is_intersecting() {
                let target = entry.target();
                let class_list = target.class_list();
                
                if class_list.contains("fade-in") || 
                   class_list.contains("slide-in-left") || 
                   class_list.contains("slide-in-right") {
                    let _ = class_list.add_1("visible");
                }
            }
        }
    }) as Box<dyn FnMut(js_sys::Array)>);
    
    let options = js_sys::Object::new();
    if js_sys::Reflect::set(&options, &"threshold".into(), &0.15.into()).is_err() ||
       js_sys::Reflect::set(&options, &"rootMargin".into(), &"0px 0px -50px 0px".into()).is_err() {
        web_sys::console::error_1(&"Failed to set observer options".into());
        return;
    }
    
    let observer = match IntersectionObserver::new_with_options(
        observer_callback.as_ref().unchecked_ref(), 
        &options.unchecked_into()
    ) {
        Ok(observer) => observer,
        Err(_) => {
            web_sys::console::error_1(&"Failed to create IntersectionObserver".into());
            return;
        }
    };
    
    if let Ok(elements) = document.query_selector_all(".fade-in, .slide-in-left, .slide-in-right") {
        for i in 0..elements.length() {
            if let Some(element) = elements.item(i) {
                if let Some(element) = element.dyn_ref::<Element>() {
                    observer.observe(element);
                }
            }
        }
    } else {
        web_sys::console::error_1(&"Failed to query animation elements".into());
    }
    
    observer_callback.forget();
}

pub fn setup_all_3d_effects() {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            // Feature cards
            if let Ok(feature_cards) = document.query_selector_all(".feature-card") {
                for i in 0..feature_cards.length() {
                    if let Some(card) = feature_cards.item(i) {
                        if let Some(element) = card.dyn_ref::<HtmlElement>() {
                            setup_card_3d_effect(element.clone(), Card3DConfig::feature_card());
                        }
                    }
                }
            }
            
            // ROI card
            if let Ok(Some(roi_card)) = document.query_selector(".roi-card") {
                if let Some(element) = roi_card.dyn_ref::<HtmlElement>() {
                    setup_card_3d_effect(element.clone(), Card3DConfig::roi_card());
                }
            }
            
            // CTA card
            if let Ok(Some(cta_card)) = document.query_selector(".cta-card") {
                if let Some(element) = cta_card.dyn_ref::<HtmlElement>() {
                    setup_card_3d_effect(element.clone(), Card3DConfig::cta_card());
                }
            }
        }
    }
}