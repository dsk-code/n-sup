use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::ui::{Container, PrimaryButton, SecondaryButton};

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="hero">
            <Container>
                <div class="hero-content fade-in">
                    <h1>
                        "製造現場の"<br /> <span class="highlight">"効率を革新"</span>
                    </h1>
                    <p>
                        "工具管理からNCプログラム管理まで、製造業務に必要な機能を一つのプラットフォームに集約。"
                        <br />
                        "AIによる支援機能で、業務効率を大幅に向上させます。"
                    </p>
                    <div class="hero-stats">
                        <div class="stat-item">
                            <div class="stat-number">"25%"</div>
                            <div class="stat-label">"生産効率向上"</div>
                        </div>
                        <div class="stat-item">
                            <div class="stat-number">"60%"</div>
                            <div class="stat-label">"工具管理時間短縮"</div>
                        </div>
                        <div class="stat-item">
                            <div class="stat-number">"40%"</div>
                            <div class="stat-label">"品質向上"</div>
                        </div>
                    </div>
                    <div class="hero-buttons">
                        <A href="/n-sup/dashboard" attr:class="btn btn-primary">
                            "ダッシュボードを開く"
                        </A>
                        <SecondaryButton href="#features">
                            "機能を見る"
                        </SecondaryButton>
                    </div>
                </div>
            </Container>
        </section>
    }
}