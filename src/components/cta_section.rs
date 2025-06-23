use leptos::prelude::*;
use leptos_router::components::A;

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
                        <div class="cta-buttons-main flex flex-col sm:flex-row gap-4 justify-center items-center mb-8">
                            <A href="/n-sup/dashboard" attr:class="w-full sm:w-auto primary-button text-center px-8 py-4 rounded-lg font-bold text-lg bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 text-white transition-all duration-300 transform hover:scale-105 shadow-lg">
                                "🚀 ダッシュボードを開く"
                            </A>
                            <A href="#features" attr:class="w-full sm:w-auto secondary-button text-center px-8 py-4 rounded-lg font-semibold border-2 border-blue-500 text-blue-400 hover:bg-blue-500 hover:text-white transition-all duration-300">
                                "機能を詳しく見る"
                            </A>
                        </div>
                        <div class="cta-buttons-grid grid grid-cols-2 md:grid-cols-3 gap-3 max-w-4xl mx-auto">
                            <A href="/n-sup/tools" attr:class="cta-link-card text-center p-4 rounded-lg bg-slate-700/30 hover:bg-slate-600/50 transition-all duration-300 border border-slate-600/50 hover:border-blue-500/50">
                                <div class="text-2xl mb-2">"🔧"</div>
                                <div class="text-sm font-medium text-white">"工具管理"</div>
                            </A>
                            <A href="/n-sup/employees" attr:class="cta-link-card text-center p-4 rounded-lg bg-slate-700/30 hover:bg-slate-600/50 transition-all duration-300 border border-slate-600/50 hover:border-purple-500/50">
                                <div class="text-2xl mb-2">"👥"</div>
                                <div class="text-sm font-medium text-white">"従業員管理"</div>
                            </A>
                            <A href="/n-sup/nc-programs" attr:class="cta-link-card text-center p-4 rounded-lg bg-slate-700/30 hover:bg-slate-600/50 transition-all duration-300 border border-slate-600/50 hover:border-cyan-500/50">
                                <div class="text-2xl mb-2">"⚙️"</div>
                                <div class="text-sm font-medium text-white">"NCプログラム"</div>
                            </A>
                            <A href="/n-sup/nc-support" attr:class="cta-link-card text-center p-4 rounded-lg bg-slate-700/30 hover:bg-slate-600/50 transition-all duration-300 border border-slate-600/50 hover:border-green-500/50">
                                <div class="text-2xl mb-2">"🛠️"</div>
                                <div class="text-sm font-medium text-white">"NC支援"</div>
                            </A>
                            <A href="/n-sup/ai-suggestions" attr:class="cta-link-card text-center p-4 rounded-lg bg-slate-700/30 hover:bg-slate-600/50 transition-all duration-300 border border-slate-600/50 hover:border-yellow-500/50">
                                <div class="text-2xl mb-2">"🤖"</div>
                                <div class="text-sm font-medium text-white">"AI提案"</div>
                            </A>
                            <A href="/n-sup/chat" attr:class="cta-link-card text-center p-4 rounded-lg bg-slate-700/30 hover:bg-slate-600/50 transition-all duration-300 border border-slate-600/50 hover:border-orange-500/50">
                                <div class="text-2xl mb-2">"💬"</div>
                                <div class="text-sm font-medium text-white">"チャット"</div>
                            </A>
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