use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct AiSuggestion {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub category: SuggestionCategory,
    pub priority: Priority,
    pub estimated_impact: String,
    pub implementation_effort: String,
    pub suggested_by: String,
    pub created_date: String,
    pub status: SuggestionStatus,
    pub tags: Vec<String>,
    pub ai_confidence: f32,
    pub roi_percentage: f32,
    pub cost_savings: u32,
    pub complexity_score: f32,
    pub strategic_value: f32,
}

#[derive(Clone, Debug)]
pub struct AIAnalytics {
    pub total_suggestions: u32,
    pub implemented_suggestions: u32,
    pub total_roi: f32,
    pub average_confidence: f32,
    pub active_initiatives: u32,
    pub projected_savings: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SuggestionCategory {
    ProcessOptimization,
    QualityImprovement,
    CostReduction,
    SafetyEnhancement,
    ProductivityIncrease,
    MaintenancePrediction,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SuggestionStatus {
    New,
    UnderReview,
    Approved,
    InProgress,
    Implemented,
    Rejected,
}

#[component]
pub fn AiToolSuggestions() -> impl IntoView {
    let (suggestions, _set_suggestions) = signal(vec![
        AiSuggestion {
            id: 1,
            title: "次世代AI工具摩耗予測システム".to_string(),
            description: "ディープラーニングとIoTセンサーを組み合わせ、工具の摩耗パターンをリアルタイムで分析。予測精度97.3%を実現し、工具寿命を最大化。".to_string(),
            category: SuggestionCategory::MaintenancePrediction,
            priority: Priority::High,
            estimated_impact: "生産効率32%向上、工具コスト28%削減、ダウンタイム85%減少".to_string(),
            implementation_effort: "2-3ヶ月（フェーズ展開）".to_string(),
            suggested_by: "Presidential AI System".to_string(),
            created_date: "2024-06-10".to_string(),
            status: SuggestionStatus::New,
            tags: vec!["ディープラーニング".to_string(), "IoT".to_string(), "予知保全".to_string(), "デジタルツイン".to_string()],
            ai_confidence: 97.3,
            roi_percentage: 340.5,
            cost_savings: 4200000,
            complexity_score: 7.2,
            strategic_value: 9.8,
        },
        AiSuggestion {
            id: 2,
            title: "量子AI品質異常検出システム".to_string(),
            description: "量子コンピュータとAIビジョンを融合した革命的品質検査システム。ナノレベルの欠陥も99.8%の精度で検出可能。".to_string(),
            category: SuggestionCategory::QualityImprovement,
            priority: Priority::High,
            estimated_impact: "不良品率87%削減、検査時間92%短縮、品質クレーム98%減".to_string(),
            implementation_effort: "1-2ヶ月（ターンキー実装）".to_string(),
            suggested_by: "Quantum AI Division".to_string(),
            created_date: "2024-06-08".to_string(),
            status: SuggestionStatus::UnderReview,
            tags: vec!["量子AI".to_string(), "ナノ検査".to_string(), "コンピュータビジョン".to_string(), "ゼロディフェクト".to_string()],
            ai_confidence: 99.8,
            roi_percentage: 485.2,
            cost_savings: 6800000,
            complexity_score: 8.9,
            strategic_value: 10.0,
        },
        AiSuggestion {
            id: 3,
            title: "生産スケジュール最適化".to_string(),
            description: "過去の生産データと現在の注文状況を分析して、最適な生産スケジュールを自動生成します。".to_string(),
            category: SuggestionCategory::ProcessOptimization,
            priority: Priority::Medium,
            estimated_impact: "生産効率15%向上、納期遵守率95%達成".to_string(),
            implementation_effort: "4-5ヶ月".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-05".to_string(),
            status: SuggestionStatus::Approved,
            tags: vec!["スケジューリング".to_string(), "最適化".to_string(), "生産管理".to_string()],
            ai_confidence: 89.4,
            roi_percentage: 245.8,
            cost_savings: 3200000,
            complexity_score: 6.5,
            strategic_value: 8.7,
        },
        AiSuggestion {
            id: 4,
            title: "スマートエネルギー最適化プラットフォーム".to_string(),
            description: "機械学習とIoTを統合し、リアルタイムでエネルギー消費を最適化。カーボンニュートラル達成に貢献。".to_string(),
            category: SuggestionCategory::CostReduction,
            priority: Priority::Medium,
            estimated_impact: "電力コスト47%削減、CO2排出量65%減少、ESG評価向上".to_string(),
            implementation_effort: "1.5-2ヶ月（AI自動展開）".to_string(),
            suggested_by: "Green AI Initiative".to_string(),
            created_date: "2024-06-03".to_string(),
            status: SuggestionStatus::InProgress,
            tags: vec!["グリーンAI".to_string(), "カーボンニュートラル".to_string(), "スマートグリッド".to_string(), "ESG".to_string()],
            ai_confidence: 92.6,
            roi_percentage: 367.4,
            cost_savings: 5400000,
            complexity_score: 5.8,
            strategic_value: 9.5,
        },
        AiSuggestion {
            id: 5,
            title: "Zero-Incident AI安全保障システム".to_string(),
            description: "次世代コンピュータビジョンと予測AIで作業現場の安全を完全保障。事故発生前の予防的介入を実現。".to_string(),
            category: SuggestionCategory::SafetyEnhancement,
            priority: Priority::High,
            estimated_impact: "労働災害100%削減、安全コンプライアンス完全達成、保険コスト90%減".to_string(),
            implementation_effort: "2-3ヶ月（緊急展開可）".to_string(),
            suggested_by: "Safety-First AI Corps".to_string(),
            created_date: "2024-06-01".to_string(),
            status: SuggestionStatus::Implemented,
            tags: vec!["ゼロ災害".to_string(), "予防AI".to_string(), "完全安全".to_string(), "コンプライアンス".to_string()],
            ai_confidence: 98.9,
            roi_percentage: 892.3,
            cost_savings: 12000000,
            complexity_score: 8.1,
            strategic_value: 10.0,
        },
    ]);

    let analytics = AIAnalytics {
        total_suggestions: suggestions.get().len() as u32,
        implemented_suggestions: suggestions.get().iter().filter(|s| s.status == SuggestionStatus::Implemented).count() as u32,
        total_roi: suggestions.get().iter().map(|s| s.roi_percentage).sum::<f32>(),
        average_confidence: suggestions.get().iter().map(|s| s.ai_confidence).sum::<f32>() / suggestions.get().len() as f32,
        active_initiatives: suggestions.get().iter().filter(|s| matches!(s.status, SuggestionStatus::InProgress | SuggestionStatus::Approved)).count() as u32,
        projected_savings: suggestions.get().iter().map(|s| s.cost_savings).sum::<u32>(),
    };

    let (selected_category, set_selected_category) = signal(None::<SuggestionCategory>);
    let (selected_priority, _set_selected_priority) = signal(None::<Priority>);
    let (show_detail_modal, set_show_detail_modal) = signal(false);
    let (viewing_suggestion, set_viewing_suggestion) = signal(None::<AiSuggestion>);

    let filtered_suggestions = move || {
        suggestions.get().into_iter().filter(|suggestion| {
            let category_match = selected_category.get()
                .is_none_or(|cat| suggestion.category == cat);
            let priority_match = selected_priority.get()
                .is_none_or(|pri| suggestion.priority == pri);
            
            category_match && priority_match
        }).collect::<Vec<_>>()
    };

    let view_suggestion = move |suggestion: AiSuggestion| {
        set_viewing_suggestion.set(Some(suggestion));
        set_show_detail_modal.set(true);
    };

    let category_text = |category: &SuggestionCategory| match category {
        SuggestionCategory::ProcessOptimization => "プロセス最適化",
        SuggestionCategory::QualityImprovement => "品質改善",
        SuggestionCategory::CostReduction => "コスト削減",
        SuggestionCategory::SafetyEnhancement => "安全性向上",
        SuggestionCategory::ProductivityIncrease => "生産性向上",
        SuggestionCategory::MaintenancePrediction => "予知保全",
    };

    let priority_text = |priority: &Priority| match priority {
        Priority::High => "高",
        Priority::Medium => "中",
        Priority::Low => "低",
    };

    let priority_color = |priority: &Priority| match priority {
        Priority::High => "bg-red-100 text-red-800",
        Priority::Medium => "bg-yellow-100 text-yellow-800",
        Priority::Low => "bg-green-100 text-green-800",
    };

    let status_text = |status: &SuggestionStatus| match status {
        SuggestionStatus::New => "新規",
        SuggestionStatus::UnderReview => "検討中",
        SuggestionStatus::Approved => "承認済",
        SuggestionStatus::InProgress => "実装中",
        SuggestionStatus::Implemented => "実装完了",
        SuggestionStatus::Rejected => "却下",
    };

    let status_color = |status: &SuggestionStatus| match status {
        SuggestionStatus::New => "bg-blue-100 text-blue-800",
        SuggestionStatus::UnderReview => "bg-yellow-100 text-yellow-800",
        SuggestionStatus::Approved => "bg-green-100 text-green-800",
        SuggestionStatus::InProgress => "bg-purple-100 text-purple-800",
        SuggestionStatus::Implemented => "bg-green-100 text-green-800",
        SuggestionStatus::Rejected => "bg-red-100 text-red-800",
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 to-slate-800 text-white">
            <div class="container mx-auto px-4 py-8">
                <div class="mb-6">
                    <a 
                        href="/n-sup/" 
                        class="inline-flex items-center text-slate-300 hover:text-white transition-colors duration-200"
                    >
                        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                        </svg>
                        "ホームに戻る"
                    </a>
                </div>

                <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 mb-8">
                    <div>
                        <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent mb-2">
                            "💡 Presidential AI提案センター"
                            <span class="text-sm bg-gradient-to-r from-gold-400 to-yellow-500 px-3 py-1 rounded-full text-black font-semibold ml-3">"EXECUTIVE INTELLIGENCE"</span>
                        </h1>
                        <p class="text-slate-300 text-sm">
                            "量子AI・機械学習による革命的改善提案と戦略的実装ロードマップ"</p>
                    </div>
                    <div class="flex gap-2">
                        <button class="bg-gradient-to-r from-green-500 to-emerald-600 hover:from-green-600 hover:to-emerald-700 px-4 py-2 rounded-lg font-semibold transition-all duration-300 text-sm">
                            "🚀 一括実装"
                        </button>
                        <button class="bg-gradient-to-r from-purple-500 to-pink-600 hover:from-purple-600 hover:to-pink-700 px-4 py-2 rounded-lg font-semibold transition-all duration-300 text-sm">
                            "📊 ROI分析"
                        </button>
                    </div>
                </div>

                // Presidential AI Analytics Dashboard
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                    <div class="bg-gradient-to-br from-blue-800/50 to-cyan-900/50 backdrop-blur-lg rounded-xl p-6 border border-blue-500/30">
                        <div class="flex items-center justify-between mb-4">
                            <div class="text-3xl">"💡"</div>
                            <div class="text-right">
                                <div class="text-2xl font-bold text-blue-400">{analytics.total_suggestions}</div>
                                <div class="text-xs text-blue-300">"AI提案総数"</div>
                            </div>
                        </div>
                        <h3 class="text-lg font-semibold text-white mb-2">"提案総数"</h3>
                        <p class="text-sm text-blue-200">"AIが生成した改善提案"</p>
                    </div>
                    
                    <div class="bg-gradient-to-br from-green-800/50 to-emerald-900/50 backdrop-blur-lg rounded-xl p-6 border border-green-500/30">
                        <div class="flex items-center justify-between mb-4">
                            <div class="text-3xl">"✅"</div>
                            <div class="text-right">
                                <div class="text-2xl font-bold text-green-400">{analytics.implemented_suggestions}</div>
                                <div class="text-xs text-green-300">"実装完了"</div>
                            </div>
                        </div>
                        <h3 class="text-lg font-semibold text-white mb-2">"実装済み"</h3>
                        <p class="text-sm text-green-200">"成功実装されたAI提案"</p>
                    </div>
                    
                    <div class="bg-gradient-to-br from-purple-800/50 to-pink-900/50 backdrop-blur-lg rounded-xl p-6 border border-purple-500/30">
                        <div class="flex items-center justify-between mb-4">
                            <div class="text-3xl">"🎯"</div>
                            <div class="text-right">
                                <div class="text-2xl font-bold text-purple-400">{format!("{:.1}%", analytics.average_confidence)}</div>
                                <div class="text-xs text-purple-300">"AI信頼度"</div>
                            </div>
                        </div>
                        <h3 class="text-lg font-semibold text-white mb-2">"AI精度"</h3>
                        <p class="text-sm text-purple-200">"平均予測信頼度"</p>
                    </div>
                    
                    <div class="bg-gradient-to-br from-orange-800/50 to-red-900/50 backdrop-blur-lg rounded-xl p-6 border border-orange-500/30">
                        <div class="flex items-center justify-between mb-4">
                            <div class="text-3xl">"💰"</div>
                            <div class="text-right">
                                <div class="text-2xl font-bold text-orange-400">"¥"{format!("{:.1}B", analytics.projected_savings as f32 / 1000000000.0)}</div>
                                <div class="text-xs text-orange-300">"予想削減効果"</div>
                            </div>
                        </div>
                        <h3 class="text-lg font-semibold text-white mb-2">"コスト効果"</h3>
                        <p class="text-sm text-orange-200">"実装時の総削減額"</p>
                    </div>
                </div>

                // Filter and Category Section
                <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border border-slate-700 mb-8">
                    <h3 class="text-lg font-semibold text-white mb-4">"🔍 スマートフィルタリング"</h3>
                    <div class="flex flex-wrap gap-3">
                        <button 
                            class={format!("px-4 py-2 rounded-lg font-medium transition-all duration-200 {}",
                                if selected_category.get().is_none() { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(None)
                        >
                            "🌟 すべて"
                        </button>
                        <button 
                            class={format!("px-4 py-2 rounded-lg font-medium transition-all duration-200 {}",
                                if selected_category.get() == Some(SuggestionCategory::MaintenancePrediction) { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(Some(SuggestionCategory::MaintenancePrediction))
                        >
                            "🔧 予知保全"
                        </button>
                        <button 
                            class={format!("px-4 py-2 rounded-lg font-medium transition-all duration-200 {}",
                                if selected_category.get() == Some(SuggestionCategory::QualityImprovement) { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(Some(SuggestionCategory::QualityImprovement))
                        >
                            "⭐ 品質改善"
                        </button>
                        <button 
                            class={format!("px-4 py-2 rounded-lg font-medium transition-all duration-200 {}",
                                if selected_category.get() == Some(SuggestionCategory::CostReduction) { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(Some(SuggestionCategory::CostReduction))
                        >
                            "💰 コスト削減"
                        </button>
                        <button 
                            class={format!("px-4 py-2 rounded-lg font-medium transition-all duration-200 {}",
                                if selected_category.get() == Some(SuggestionCategory::SafetyEnhancement) { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(Some(SuggestionCategory::SafetyEnhancement))
                        >
                            "🛡️ 安全性向上"
                        </button>
                        <button 
                            class={format!("px-4 py-2 rounded-lg font-medium transition-all duration-200 {}",
                                if selected_category.get() == Some(SuggestionCategory::ProcessOptimization) { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(Some(SuggestionCategory::ProcessOptimization))
                        >
                            "⚡ プロセス最適化"
                        </button>
                    </div>
                </div>

                // AI Suggestions Grid
                <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6 mb-8">
                    <For
                        each=filtered_suggestions
                        key=|suggestion| suggestion.id
                        children=move |suggestion| {
                            let suggestion_for_view = suggestion.clone();
                            let category_icon = match suggestion.category {
                                SuggestionCategory::MaintenancePrediction => "🔧",
                                SuggestionCategory::QualityImprovement => "⭐",
                                SuggestionCategory::CostReduction => "💰",
                                SuggestionCategory::SafetyEnhancement => "🛡️",
                                SuggestionCategory::ProcessOptimization => "⚡",
                                SuggestionCategory::ProductivityIncrease => "🚀",
                            };
                            let priority_icon = match suggestion.priority {
                                Priority::High => "🔥",
                                Priority::Medium => "🟡",
                                Priority::Low => "🟢",
                            };
                            let status_bg = match suggestion.status {
                                SuggestionStatus::New => "from-blue-800/30 to-blue-900/30 border-blue-500/30",
                                SuggestionStatus::UnderReview => "from-yellow-800/30 to-yellow-900/30 border-yellow-500/30",
                                SuggestionStatus::Approved => "from-green-800/30 to-green-900/30 border-green-500/30",
                                SuggestionStatus::InProgress => "from-purple-800/30 to-purple-900/30 border-purple-500/30",
                                SuggestionStatus::Implemented => "from-emerald-800/30 to-emerald-900/30 border-emerald-500/30",
                                SuggestionStatus::Rejected => "from-red-800/30 to-red-900/30 border-red-500/30",
                            };
                            
                            view! {
                                <div class={format!("bg-gradient-to-br {} backdrop-blur-lg rounded-xl p-6 border transition-all duration-300 hover:scale-105 cursor-pointer", status_bg)}
                                     on:click=move |_| view_suggestion(suggestion_for_view.clone())>
                                    <div class="flex items-start justify-between mb-4">
                                        <div class="flex gap-2">
                                            <div class="text-2xl">{category_icon}</div>
                                            <div class="text-lg">{priority_icon}</div>
                                        </div>
                                        <div class="text-right">
                                            <div class="text-lg font-bold text-green-400">{format!("{:.1}%", suggestion.ai_confidence)}</div>
                                            <div class="text-xs text-slate-400">"AI信頼度"</div>
                                        </div>
                                    </div>
                                    
                                    <h3 class="text-lg font-semibold text-white mb-3">{suggestion.title.clone()}</h3>
                                    <p class="text-sm text-slate-300 mb-4 line-clamp-3">{suggestion.description.clone()}</p>
                                    
                                    <div class="grid grid-cols-2 gap-3 mb-4">
                                        <div class="bg-slate-700/30 p-3 rounded-lg text-center">
                                            <div class="text-lg font-bold text-blue-400">{format!("+{:.0}%", suggestion.roi_percentage)}</div>
                                            <div class="text-xs text-slate-400">"ROI"</div>
                                        </div>
                                        <div class="bg-slate-700/30 p-3 rounded-lg text-center">
                                            <div class="text-lg font-bold text-green-400">"¥"{format!("{:.1}M", suggestion.cost_savings as f32 / 1000000.0)}</div>
                                            <div class="text-xs text-slate-400">"削減効果"</div>
                                        </div>
                                    </div>
                                    
                                    <div class="flex flex-wrap gap-1 mb-4">
                                        <For
                                            each=move || suggestion.tags.clone()
                                            key=|tag| tag.clone()
                                            children=move |tag| {
                                                view! {
                                                    <span class="px-2 py-1 bg-blue-600/20 border border-blue-500/30 text-blue-300 rounded text-xs">
                                                        {tag}
                                                    </span>
                                                }
                                            }
                                        />
                                    </div>
                                    
                                    <div class="flex justify-between items-center">
                                        <span class={format!("px-3 py-1 rounded-full text-xs font-medium {}", status_color(&suggestion.status))}>
                                            {status_text(&suggestion.status)}
                                        </span>
                                        <div class="text-xs text-slate-400">{suggestion.implementation_effort.clone()}</div>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}