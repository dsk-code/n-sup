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
                        <div class="cta-buttons flex flex-wrap gap-3 justify-center items-center">
                            <A href="/n-sup/tools" attr:class="w-full sm:w-auto primary-button text-center px-6 py-3 rounded-lg font-semibold bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 text-white transition-all duration-300 transform hover:scale-105">
                                "工具管理"
                            </A>
                            <A href="/n-sup/employees" attr:class="w-full sm:w-auto secondary-button text-center px-6 py-3 rounded-lg font-semibold border-2 border-blue-500 text-blue-400 hover:bg-blue-500 hover:text-white transition-all duration-300">
                                "従業員管理"
                            </A>
                            <A href="/n-sup/nc-programs" attr:class="w-full sm:w-auto secondary-button text-center px-6 py-3 rounded-lg font-semibold border-2 border-purple-500 text-purple-400 hover:bg-purple-500 hover:text-white transition-all duration-300">
                                "NCプログラム管理"
                            </A>
                            <A href="/n-sup/nc-support" attr:class="w-full sm:w-auto secondary-button text-center px-6 py-3 rounded-lg font-semibold border-2 border-cyan-500 text-cyan-400 hover:bg-cyan-500 hover:text-white transition-all duration-300">
                                "NCプログラム支援"
                            </A>
                            <A href="/n-sup/chat" attr:class="w-full sm:w-auto secondary-button text-center px-6 py-3 rounded-lg font-semibold border-2 border-green-500 text-green-400 hover:bg-green-500 hover:text-white transition-all duration-300">
                                "チャット"
                            </A>
                            <A href="/n-sup/ai-suggestions" attr:class="w-full sm:w-auto secondary-button text-center px-6 py-3 rounded-lg font-semibold border-2 border-yellow-500 text-yellow-400 hover:bg-yellow-500 hover:text-white transition-all duration-300">
                                "AI工具提案"
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