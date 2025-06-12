use leptos::prelude::*;

#[derive(Clone)]
pub struct Benefit {
    pub number: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[component]
pub fn BenefitsSection() -> impl IntoView {
    let benefits = vec![
        Benefit {
            number: "01",
            title: "業務効率 60% 向上",
            description:
                "工具の検索時間を削減し、NCプログラムの共有を効率化。従業員の作業時間を大幅に短縮。",
        },
        Benefit {
            number: "02",
            title: "コスト削減 30%",
            description:
                "無駄な工具購入を防ぎ、最適な工具提案によりコストを削減。ROIは3ヶ月で実現。",
        },
        Benefit {
            number: "03",
            title: "品質向上",
            description: "標準化されたNCプログラム管理により、製品品質の一貫性を保証。",
        },
    ];

    view! {
        <section id="benefits" class="benefits">
            <div class="container">
                <div class="section-header fade-in">
                    <h2>"NSup導入のメリット"</h2>
                    <p>
                        "業務効率化からコスト削減まで、具体的な成果を実現"
                    </p>
                </div>
                <div class="benefits-content">
                    <div class="benefits-list">
                        <For
                            each=move || benefits.clone()
                            key=|benefit| benefit.number
                            children=move |benefit| {
                                view! { <BenefitItem benefit=benefit /> }
                            }
                        />
                    </div>
                    <RoiCard />
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn BenefitItem(benefit: Benefit) -> impl IntoView {
    view! {
        <div class="benefit-item slide-in-left">
            <div class="benefit-number">{benefit.number}</div>
            <div class="benefit-content">
                <h3>{benefit.title}</h3>
                <p>{benefit.description}</p>
            </div>
        </div>
    }
}

#[component]
pub fn RoiCard() -> impl IntoView {
    view! {
        <div class="roi-card slide-in-right">
            <div class="roi-card-content">
                <div class="roi-number">"3ヶ月"</div>
                <div class="roi-text">"で投資回収"</div>
                <div class="roi-subtext">"平均的な導入効果"</div>
                <div class="roi-stats">
                    <div class="roi-stat">
                        <div class="roi-stat-number">"60%"</div>
                        <div class="roi-stat-label">"効率向上"</div>
                    </div>
                    <div class="roi-stat">
                        <div class="roi-stat-number">"30%"</div>
                        <div class="roi-stat-label">"コスト削減"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}