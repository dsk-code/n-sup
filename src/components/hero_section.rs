use leptos::prelude::*;
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
                    <div class="hero-buttons">
                        <PrimaryButton href="#cta">
                            "今すぐ始める"
                        </PrimaryButton>
                        <SecondaryButton href="#features">
                            "デモを見る"
                        </SecondaryButton>
                    </div>
                </div>
            </Container>
        </section>
    }
}