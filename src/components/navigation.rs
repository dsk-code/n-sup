use leptos::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 right-0 z-50 bg-slate-900/80 backdrop-blur-md border-b border-slate-700/50">
            <div class="container mx-auto px-4">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center space-x-8">
                        <a href="/" class="text-xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent hover:scale-105 transition-transform duration-200">
                            "NSup"
                        </a>
                        
                        <div class="hidden md:flex space-x-6">
                            <a href="/" class="text-slate-300 hover:text-white transition-colors duration-200">
                                "ホーム"
                            </a>
                            <a href="/tools" class="text-slate-300 hover:text-white transition-colors duration-200">
                                "工具管理"
                            </a>
                            <a href="/employees" class="text-slate-300 hover:text-white transition-colors duration-200">
                                "従業員管理"
                            </a>
                        </div>
                    </div>
                    
                    <div class="hidden md:flex items-center space-x-4">
                        <button class="text-slate-300 hover:text-white transition-colors duration-200">
                            "ログイン"
                        </button>
                        <button class="bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 px-4 py-2 rounded-lg font-medium transition-all duration-300 transform hover:scale-105">
                            "無料トライアル"
                        </button>
                    </div>
                    
                    // Mobile menu button
                    <div class="md:hidden">
                        <button class="text-slate-300 hover:text-white">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn BackToHome() -> impl IntoView {
    view! {
        <div class="mb-6">
            <a 
                href="/" 
                class="inline-flex items-center text-slate-300 hover:text-white transition-colors duration-200"
            >
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                </svg>
                "ホームに戻る"
            </a>
        </div>
    }
}