use web_sys::*;
use wasm_bindgen::prelude::*;

pub trait ScrollHandler {
    fn handle_scroll(&self, scroll_y: f64, document: &Document);
}

pub struct HeaderScrollHandler;

impl ScrollHandler for HeaderScrollHandler {
    fn handle_scroll(&self, scroll_y: f64, document: &Document) {
        if let Ok(Some(header)) = document.query_selector(".header") {
            if scroll_y > 80.0 {
                let _ = header.set_attribute("style", 
                    "background: linear-gradient(145deg, rgba(15, 23, 42, 0.95), rgba(30, 41, 59, 0.9)); \
                     box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);"
                );
            } else {
                let _ = header.set_attribute("style", 
                    "background: linear-gradient(145deg, rgba(15, 23, 42, 0.9), rgba(30, 41, 59, 0.8)); \
                     box-shadow: 0 4px 32px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);"
                );
            }
        }
    }
}

pub struct ParallaxScrollHandler;

impl ScrollHandler for ParallaxScrollHandler {
    fn handle_scroll(&self, scroll_y: f64, document: &Document) {
        if let Ok(Some(ambient_bg)) = document.query_selector(".ambient-bg") {
            let parallax = scroll_y * 0.3;
            let _ = ambient_bg.set_attribute("style", 
                &format!("transform: translateY({}px) translateZ(-10px);", parallax)
            );
        }
        
        if let Ok(Some(subtle_grid)) = document.query_selector(".subtle-grid") {
            let grid_parallax = scroll_y * 0.1;
            let _ = subtle_grid.set_attribute("style", 
                &format!("transform: translateY({}px) translateZ(-5px);", grid_parallax)
            );
        }
    }
}

pub fn setup_scroll_handler<T: ScrollHandler + 'static>(handler: T) {
    let scroll_closure = Closure::wrap(Box::new(move || {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                let scroll_y = window.scroll_y().unwrap_or(0.0);
                handler.handle_scroll(scroll_y, &document);
            }
        }
    }) as Box<dyn FnMut()>);
    
    if let Some(window) = window() {
        let _ = window.add_event_listener_with_callback("scroll", scroll_closure.as_ref().unchecked_ref());
    }
    scroll_closure.forget();
}

pub fn setup_all_scroll_handlers() {
    setup_scroll_handler(HeaderScrollHandler);
    setup_scroll_handler(ParallaxScrollHandler);
}

pub fn setup_smooth_scrolling() {
    // CSS already provides scroll-behavior: smooth
    // This function is kept for future enhancements
}