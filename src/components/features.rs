use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::ui::{Container, SectionHeader, AnimatedCard};

#[derive(Clone)]
pub struct Feature {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[component]
pub fn FeaturesSection() -> impl IntoView {
    let features = vec![
        Feature {
            icon: "📊",
            title: "ダッシュボード",
            description: "リアルタイムデータとKPIを一目で確認。生産状況を即座に把握できます。",
        },
        Feature {
            icon: "🔧",
            title: "工具管理",
            description: "リアルタイムでの工具在庫管理と追跡。消耗品の自動アラート機能付き。",
        },
        Feature {
            icon: "👥",
            title: "従業員別管理",
            description: "各従業員の工具所持状況を個別管理。責任の明確化と効率化を実現。",
        },
        Feature {
            icon: "⚙️",
            title: "NCプログラム管理",
            description: "バージョン管理機能付きのNCプログラム保管・共有システム。",
        },
        Feature {
            icon: "🤖",
            title: "AI工具提案",
            description: "機械学習による最適な工具の提案と推奨。コスト削減と効率向上。",
        },
        Feature {
            icon: "🛠️",
            title: "NCプログラム支援",
            description: "AIによるNCプログラムの最適化提案と自動コード生成支援。",
        },
        Feature {
            icon: "💬",
            title: "チャット機能",
            description: "チーム内のリアルタイムコミュニケーション。プロジェクト単位でのやり取りが可能。",
        },
    ];

    view! {
        <section id="features" class="features">
            <Container>
                <SectionHeader 
                    title="包括的な機能セット"
                    description="製造現場で必要な全ての機能を、直感的で使いやすいインターフェースで提供"
                />
                <div class="features-grid">
                    <For
                        each=move || features.clone()
                        key=|feature| feature.title
                        children=move |feature| {
                            view! { <FeatureCard feature=feature /> }
                        }
                    />
                </div>
            </Container>
        </section>
    }
}

#[component]
pub fn FeatureCard(feature: Feature) -> impl IntoView {
    let link_url = match feature.title {
        "ダッシュボード" => Some("/n-sup/dashboard"),
        "工具管理" => Some("/n-sup/tools"),
        "従業員別管理" => Some("/n-sup/employees"),
        "NCプログラム管理" => Some("/n-sup/nc-programs"),
        "チャット機能" => Some("/n-sup/chat"),
        "AI工具提案" => Some("/n-sup/ai-suggestions"),
        "NCプログラム支援" => Some("/n-sup/nc-support"),
        _ => None,
    };

    let card_content = view! {
        <div class="feature-card-content">
            <span class="feature-icon">{feature.icon}</span>
            <h3>{feature.title}</h3>
            <p>{feature.description}</p>
        </div>
    };

    view! {
        <AnimatedCard class="feature-card" animation_class="fade-in">
            {
                if let Some(url) = link_url {
                    view! {
                        <A href=url attr:class="feature-card-link block h-full hover:transform hover:scale-105 transition-all duration-300">
                            {card_content}
                        </A>
                    }.into_any()
                } else {
                    card_content.into_any()
                }
            }
        </AnimatedCard>
    }
}