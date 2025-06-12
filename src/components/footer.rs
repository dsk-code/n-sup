use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    let links = vec![
        "プライバシーポリシー",
        "利用規約",
        "お問い合わせ",
        "サポート",
    ];

    view! {
        <footer class="footer">
            <div class="container">
                <div class="footer-logo">"NSup"</div>
                <div class="footer-description">
                    "製造現場の効率化を支援する包括的なプラットフォーム"
                </div>
                <div class="footer-links">
                    <For
                        each=move || links.clone()
                        key=|link| *link
                        children=move |link| {
                            view! { <a href="#">{link}</a> }
                        }
                    />
                </div>
                <div class="footer-copyright">"© 2025 NSup. All rights reserved."</div>
            </div>
        </footer>
    }
}