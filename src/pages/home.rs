use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white">
            <Header />
            <main>
                <HeroSection />
                <FeaturesSection />
                <StatsSection />
                <ActivitySection />
            </main>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    let is_search_open = RwSignal::new(false);

    view! {
        <header class="fixed top-0 left-0 right-0 z-50 bg-white/80 backdrop-blur-xl border-b border-gray-100">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-14 sm:h-16">
                    <div class="flex items-center space-x-3 sm:space-x-4">
                        <span class="text-lg sm:text-xl font-semibold text-gray-900">"NSup"</span>
                    </div>

                    <nav class="hidden lg:flex items-center space-x-8 xl:space-x-10">
                        <a
                            href="#"
                            class="relative text-gray-600 hover:text-black font-medium transition-all duration-300 px-3 py-2 rounded-lg hover:bg-gray-50 group text-sm xl:text-base"
                        >
                            "ダッシュボード"
                            <span class="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-0 h-0.5 bg-black transition-all duration-300 group-hover:w-3/4"></span>
                        </a>
                        <a
                            href="#"
                            class="relative text-gray-600 hover:text-black font-medium transition-all duration-300 px-3 py-2 rounded-lg hover:bg-gray-50 group text-sm xl:text-base"
                        >
                            "工具管理"
                            <span class="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-0 h-0.5 bg-black transition-all duration-300 group-hover:w-3/4"></span>
                        </a>
                        <a
                            href="#"
                            class="relative text-gray-600 hover:text-black font-medium transition-all duration-300 px-3 py-2 rounded-lg hover:bg-gray-50 group text-sm xl:text-base"
                        >
                            "NCプログラム"
                            <span class="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-0 h-0.5 bg-black transition-all duration-300 group-hover:w-3/4"></span>
                        </a>
                        <a
                            href="#"
                            class="relative text-gray-600 hover:text-black font-medium transition-all duration-300 px-3 py-2 rounded-lg hover:bg-gray-50 group text-sm xl:text-base"
                        >
                            "チャット"
                            <span class="absolute bottom-0 left-1/2 transform -translate-x-1/2 w-0 h-0.5 bg-black transition-all duration-300 group-hover:w-3/4"></span>
                        </a>
                    </nav>

                    <div class="flex items-center space-x-2 sm:space-x-4">
                        <Show
                            when=move || is_search_open.get()
                            fallback=move || {
                                view! {
                                    <button
                                        class="p-2 text-gray-600 hover:text-gray-900 transition-colors duration-200"
                                        on:click=move |_| is_search_open.set(true)
                                    >
                                        <svg
                                            class="w-4 h-4 sm:w-5 sm:h-5"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="1.5"
                                                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                                            />
                                        </svg>
                                    </button>
                                }
                            }
                        >
                            <div class="flex items-center bg-gray-100 rounded-full px-4 py-2 w-48 sm:w-64">
                                <input
                                    type="text"
                                    placeholder="検索..."
                                    class="bg-transparent outline-none flex-1 text-sm text-gray-900 placeholder-gray-500"
                                    autofocus
                                />
                                <button
                                    class="ml-2 text-gray-500 hover:text-gray-700"
                                    on:click=move |_| is_search_open.set(false)
                                >
                                    <svg
                                        class="w-4 h-4"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M6 18L18 6M6 6l12 12"
                                        />
                                    </svg>
                                </button>
                            </div>
                        </Show>

                        <button class="lg:hidden p-2 text-gray-600 hover:text-gray-900">
                            <svg
                                class="w-5 h-5"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 6h16M4 12h16M4 18h16"
                                />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </header>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <section class="pt-20 sm:pt-24 lg:pt-32 pb-12 sm:pb-16 lg:pb-20 px-4 sm:px-6 lg:px-8">
            <div class="max-w-4xl mx-auto text-center">
                <h1 class="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold text-gray-900 mb-4 sm:mb-6 tracking-tight leading-none">
                    "工具管理の" <br class="sm:hidden" />
                    <span class="block sm:inline">"未来を今に"</span>
                </h1>
                <p class="text-base sm:text-lg md:text-xl lg:text-2xl text-gray-600 mb-8 sm:mb-10 lg:mb-12 leading-relaxed font-light max-w-3xl mx-auto px-4 sm:px-0">
                    "シンプルで美しいインターフェースで、工具管理からNCプログラムまで。"
                    <br class="hidden sm:block" />
                    "あらゆる製造業務を効率化します。"
                </p>
                <div class="flex flex-col sm:flex-row justify-center items-center space-y-3 sm:space-y-0 sm:space-x-4 lg:space-x-6 px-4 sm:px-0">
                    <button class="w-full sm:w-auto bg-black text-white px-6 sm:px-8 py-3 sm:py-4 rounded-full font-medium hover:bg-gray-800 transition-all duration-300 transform hover:scale-105 text-sm sm:text-base">
                        "今すぐ始める"
                    </button>
                    <button class="w-full sm:w-auto text-black border border-gray-300 px-6 sm:px-8 py-3 sm:py-4 rounded-full font-medium hover:border-gray-400 transition-all duration-300 text-sm sm:text-base">
                        "デモを見る"
                    </button>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeaturesSection() -> impl IntoView {
    view! {
        <section class="py-12 sm:py-16 lg:py-20 px-4 sm:px-6 lg:px-8 bg-gray-50">
            <div class="max-w-7xl mx-auto">
                <div class="text-center mb-12 sm:mb-16">
                    <h2 class="text-3xl sm:text-4xl lg:text-5xl font-bold text-gray-900 mb-3 sm:mb-4">
                        "すべてがシンプルに"
                    </h2>
                    <p class="text-lg sm:text-xl text-gray-600 font-light">
                        "複雑な作業も、直感的な操作で"
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-6 lg:gap-8">
                    <FeatureCard
                        title="工具管理"
                        description="在庫から配布まで、すべての工具を一元管理。リアルタイムで状況を把握できます。"
                        icon_bg="bg-blue-500"
                        icon="🔧"
                    />
                    <FeatureCard
                        title="従業員管理"
                        description="誰がどの工具を使用しているかを瞬時に確認。効率的な人材配置をサポートします。"
                        icon_bg="bg-green-500"
                        icon="👥"
                    />
                    <FeatureCard
                        title="NCプログラム"
                        description="プログラムの作成から管理まで。バージョン管理も含めて完全サポート。"
                        icon_bg="bg-purple-500"
                        icon="💻"
                    />
                    <FeatureCard
                        title="チーム連携"
                        description="リアルタイムチャットで情報共有。必要な時に必要な人とすぐに連絡を取れます。"
                        icon_bg="bg-orange-500"
                        icon="💬"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    title: &'static str,
    description: &'static str,
    icon_bg: &'static str,
    icon: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-2xl sm:rounded-3xl p-6 sm:p-8 shadow-sm hover:shadow-lg transition-all duration-500 cursor-pointer group">
            <div class=format!(
                "w-12 h-12 sm:w-14 sm:h-14 rounded-xl sm:rounded-2xl flex items-center justify-center text-xl sm:text-2xl mb-4 sm:mb-6 {}",
                icon_bg,
            )>{icon}</div>
            <h3 class="text-xl sm:text-2xl font-bold text-gray-900 mb-3 sm:mb-4">{title}</h3>
            <p class="text-gray-600 leading-relaxed font-light text-sm sm:text-base">
                {description}
            </p>
            <div class="mt-4 sm:mt-6 flex items-center text-black font-medium group-hover:translate-x-2 transition-transform duration-300 text-sm sm:text-base">
                "詳しく見る"
                <svg
                    class="w-4 h-4 sm:w-5 sm:h-5 ml-2"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M17 8l4 4m0 0l-4 4m4-4H3"
                    />
                </svg>
            </div>
        </div>
    }
}

#[component]
fn StatsSection() -> impl IntoView {
    view! {
        <section class="py-12 sm:py-16 lg:py-20 px-4 sm:px-6 lg:px-8">
            <div class="max-w-7xl mx-auto">
                <div class="bg-black rounded-2xl sm:rounded-3xl p-8 sm:p-12 lg:p-16 text-center">
                    <h2 class="text-3xl sm:text-4xl lg:text-5xl font-bold text-white mb-3 sm:mb-4">
                        "数字で見るNSup"
                    </h2>
                    <p class="text-lg sm:text-xl text-gray-300 mb-8 sm:mb-10 lg:mb-12 font-light">
                        "効率性を実現する確かな実績"
                    </p>

                    <div class="grid grid-cols-1 sm:grid-cols-3 gap-8 sm:gap-6 lg:gap-12">
                        <StatItem
                            number="247"
                            label="管理中の工具"
                            description="リアルタイム追跡"
                        />
                        <StatItem
                            number="18"
                            label="アクティブユーザー"
                            description="同時接続可能"
                        />
                        <StatItem
                            number="89"
                            label="NCプログラム"
                            description="バージョン管理済み"
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn StatItem(number: &'static str, label: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="text-center py-4 sm:py-0">
            <div class="text-4xl sm:text-5xl lg:text-6xl font-bold text-white mb-2">{number}</div>
            <div class="text-lg sm:text-xl text-gray-300 font-medium mb-1">{label}</div>
            <div class="text-sm text-gray-400 font-light">{description}</div>
        </div>
    }
}

#[component]
fn ActivitySection() -> impl IntoView {
    view! {
        <section class="py-12 sm:py-16 lg:py-20 px-4 sm:px-6 lg:px-8 bg-gray-50">
            <div class="max-w-4xl mx-auto">
                <div class="text-center mb-8 sm:mb-12">
                    <h2 class="text-3xl sm:text-4xl font-bold text-gray-900 mb-3 sm:mb-4">
                        "リアルタイム活動"
                    </h2>
                    <p class="text-lg sm:text-xl text-gray-600 font-light">
                        "システム内のすべての動きを把握"
                    </p>
                </div>

                <div class="bg-white rounded-2xl sm:rounded-3xl p-6 sm:p-8 shadow-sm">
                    <div class="space-y-4 sm:space-y-6">
                        <ActivityItem
                            title="新しい工具が登録されました"
                            description="エンドミル φ10mm - SKH材"
                            time="2分前"
                            icon="🔧"
                            color="bg-blue-50"
                        />
                        <ActivityItem
                            title="田中さんに工具が割り当てられました"
                            description="ドリル φ8mm × 3本"
                            time="15分前"
                            icon="👤"
                            color="bg-green-50"
                        />
                        <ActivityItem
                            title="NCプログラムが更新されました"
                            description="Part-A-Rev3.nc"
                            time="1時間前"
                            icon="💻"
                            color="bg-purple-50"
                        />
                        <ActivityItem
                            title="新しいメッセージ"
                            description="山田さんからの工具リクエスト"
                            time="2時間前"
                            icon="💬"
                            color="bg-orange-50"
                        />
                    </div>

                    <div class="mt-6 sm:mt-8 text-center">
                        <button class="text-black font-medium hover:text-gray-600 transition-colors duration-200 text-sm sm:text-base">
                            "すべての活動を表示"
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ActivityItem(
    title: &'static str,
    description: &'static str,
    time: &'static str,
    icon: &'static str,
    color: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex items-center p-3 sm:p-4 hover:bg-gray-50 rounded-xl sm:rounded-2xl transition-all duration-200 cursor-pointer">
            <div class=format!(
                "w-10 h-10 sm:w-12 sm:h-12 rounded-full flex items-center justify-center text-lg sm:text-xl mr-3 sm:mr-4 flex-shrink-0 {}",
                color,
            )>{icon}</div>
            <div class="flex-1 min-w-0">
                <h4 class="font-semibold text-gray-900 mb-1 text-sm sm:text-base truncate">
                    {title}
                </h4>
                <p class="text-gray-600 font-light text-xs sm:text-sm truncate">{description}</p>
            </div>
            <div class="text-xs sm:text-sm text-gray-400 font-light ml-2 flex-shrink-0">{time}</div>
        </div>
    }
}
