use leptos::prelude::*;

#[component]
pub fn CtaSection() -> impl IntoView {
    let features = vec![
        "クレジットカード不要",
        "14日間無料",
        "いつでもキャンセル可能",
    ];

    view! {
        <section id="cta" class="cta-section">
            <div class="container">
                <div class="cta-card fade-in">
                    <div class="cta-card-content">
                        <h2>"今すぐNSupで業務を革新"</h2>
                        <p>
                            "14日間の無料トライアルで、NSup の効果を実際に体験してください。"
                            <br /> "導入サポートも充実しています。"
                        </p>
                        <div class="cta-buttons">
                            <a href="#" class="primary-button">
                                "無料トライアル開始"
                            </a>
                            <a href="#" class="secondary-button">
                                "資料請求"
                            </a>
                        </div>
                        <div class="cta-features">
                            <For
                                each=move || features.clone()
                                key=|feature| *feature
                                children=move |feature| {
                                    view! {
                                        <div class="cta-feature">
                                            <span class="checkmark">"✓"</span>
                                            <span>{feature}</span>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}