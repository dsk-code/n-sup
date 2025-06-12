use leptos::prelude::*;

#[component]
pub fn PrimaryButton(
    #[prop(into)] href: String,
    children: Children,
) -> impl IntoView {
    view! {
        <a href=href class="primary-button">
            {children()}
        </a>
    }
}

#[component]
pub fn SecondaryButton(
    #[prop(into)] href: String,
    children: Children,
) -> impl IntoView {
    view! {
        <a href=href class="secondary-button">
            {children()}
        </a>
    }
}

#[component]
pub fn SectionHeader(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <div class="section-header fade-in">
            <h2>{title}</h2>
            <p>{description}</p>
        </div>
    }
}

#[component]
pub fn AnimatedCard(
    #[prop(into)] class: String,
    #[prop(into)] animation_class: String,
    children: Children,
) -> impl IntoView {
    let combined_classes = format!("{} {}", class, animation_class);
    
    view! {
        <div class=combined_classes>
            {children()}
        </div>
    }
}

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! {
        <div class="container">
            {children()}
        </div>
    }
}