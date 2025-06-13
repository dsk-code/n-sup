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
    let (suggestions, set_suggestions) = signal(vec![
        AiSuggestion {
            id: 1,
            title: "工具摩耗予測システム".to_string(),
            description: "機械学習を使用して工具の摩耗パターンを分析し、最適な交換タイミングを予測します。これにより工具寿命を最大化し、突発的な故障を防げます。".to_string(),
            category: SuggestionCategory::MaintenancePrediction,
            priority: Priority::High,
            estimated_impact: "生産効率20%向上、工具コスト15%削減".to_string(),
            implementation_effort: "3-4ヶ月".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-10".to_string(),
            status: SuggestionStatus::New,
            tags: vec!["機械学習".to_string(), "予知保全".to_string(), "工具管理".to_string()],
        },
        AiSuggestion {
            id: 2,
            title: "品質異常検出の自動化".to_string(),
            description: "コンピュータビジョンを活用して製品の表面欠陥や寸法異常を自動検出し、不良品の流出を防止します。".to_string(),
            category: SuggestionCategory::QualityImprovement,
            priority: Priority::High,
            estimated_impact: "不良品率50%削減、検査時間60%短縮".to_string(),
            implementation_effort: "2-3ヶ月".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-08".to_string(),
            status: SuggestionStatus::UnderReview,
            tags: vec!["画像認識".to_string(), "品質管理".to_string(), "自動化".to_string()],
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
        },
        AiSuggestion {
            id: 4,
            title: "エネルギー消費最適化".to_string(),
            description: "機械の稼働パターンを分析し、電力消費を最小化する運転プランを提案します。".to_string(),
            category: SuggestionCategory::CostReduction,
            priority: Priority::Medium,
            estimated_impact: "電力コスト25%削減、CO2排出量20%減少".to_string(),
            implementation_effort: "2-3ヶ月".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-03".to_string(),
            status: SuggestionStatus::InProgress,
            tags: vec!["省エネ".to_string(), "コスト削減".to_string(), "環境".to_string()],
        },
        AiSuggestion {
            id: 5,
            title: "作業者安全監視システム".to_string(),
            description: "カメラとセンサーを使用して作業者の安全を監視し、危険な状況を検知して警告を発します。".to_string(),
            category: SuggestionCategory::SafetyEnhancement,
            priority: Priority::High,
            estimated_impact: "労働災害90%削減、安全意識向上".to_string(),
            implementation_effort: "3-4ヶ月".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-01".to_string(),
            status: SuggestionStatus::Implemented,
            tags: vec!["安全管理".to_string(), "監視システム".to_string(), "AI".to_string()],
        },
    ]);

    let (selected_category, set_selected_category) = signal(None::<SuggestionCategory>);
    let (selected_priority, set_selected_priority) = signal(None::<Priority>);
    let (show_detail_modal, set_show_detail_modal) = signal(false);
    let (viewing_suggestion, set_viewing_suggestion) = signal(None::<AiSuggestion>);

    let filtered_suggestions = move || {
        suggestions.get().into_iter().filter(|suggestion| {
            let category_match = selected_category.get()
                .map_or(true, |cat| suggestion.category == cat);
            let priority_match = selected_priority.get()
                .map_or(true, |pri| suggestion.priority == pri);
            
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
                        href="/" 
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
                            "AI工具提案"
                        </h1>
                        <p class="text-slate-300 text-sm">
                            "AIが分析した改善提案と効率化アイデア"
                        </p>
                    </div>
                    <div class="flex gap-2 text-sm">
                        <div class="bg-slate-800/50 backdrop-blur-sm rounded-lg p-3">
                            <span class="text-slate-400">"提案数: "</span>
                            <span class="text-white font-semibold">{move || suggestions.get().len()}</span>
                        </div>
                    </div>
                </div>

                // Filters
                <div class="mb-6 flex flex-wrap gap-4">
                    <div class="flex flex-wrap gap-2">
                        <span class="text-slate-400 text-sm py-2">カテゴリ:</span>
                        <button 
                            class={move || format!("px-3 py-1 rounded-full text-xs font-medium transition-colors duration-200 {}",
                                if selected_category.get().is_none() { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(None)
                        >
                            "すべて"
                        </button>
                        <button 
                            class={move || format!("px-3 py-1 rounded-full text-xs font-medium transition-colors duration-200 {}",
                                if selected_category.get() == Some(SuggestionCategory::MaintenancePrediction) { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(Some(SuggestionCategory::MaintenancePrediction))
                        >
                            "予知保全"
                        </button>
                        <button 
                            class={move || format!("px-3 py-1 rounded-full text-xs font-medium transition-colors duration-200 {}",
                                if selected_category.get() == Some(SuggestionCategory::QualityImprovement) { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(Some(SuggestionCategory::QualityImprovement))
                        >
                            "品質改善"
                        </button>
                        <button 
                            class={move || format!("px-3 py-1 rounded-full text-xs font-medium transition-colors duration-200 {}",
                                if selected_category.get() == Some(SuggestionCategory::ProcessOptimization) { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(Some(SuggestionCategory::ProcessOptimization))
                        >
                            "プロセス最適化"
                        </button>
                        <button 
                            class={move || format!("px-3 py-1 rounded-full text-xs font-medium transition-colors duration-200 {}",
                                if selected_category.get() == Some(SuggestionCategory::SafetyEnhancement) { "bg-blue-500 text-white" } else { "bg-slate-700 text-slate-300 hover:bg-slate-600" }
                            )}
                            on:click=move |_| set_selected_category.set(Some(SuggestionCategory::SafetyEnhancement))
                        >
                            "安全性向上"
                        </button>
                    </div>
                </div>

                // Desktop Table View
                <div class="hidden lg:block bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 shadow-xl overflow-hidden">
                    <div class="overflow-x-auto">
                        <table class="w-full">
                            <thead class="bg-slate-700/50">
                                <tr>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"提案タイトル"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"カテゴリ"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"優先度"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"ステータス"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"予想効果"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"実装期間"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"操作"</th>
                                </tr>
                            </thead>
                            <tbody class="divide-y divide-slate-700/50">
                                <For
                                    each=filtered_suggestions
                                    key=|suggestion| suggestion.id
                                    children=move |suggestion| {
                                        let suggestion_for_view = suggestion.clone();
                                        view! {
                                            <tr class="hover:bg-slate-700/30 transition-colors duration-200">
                                                <td class="px-4 py-3">
                                                    <div>
                                                        <div class="font-medium text-white">{suggestion.title.clone()}</div>
                                                        <div class="text-slate-400 text-xs mt-1 line-clamp-2">{suggestion.description.clone().chars().take(80).collect::<String>()}"..."</div>
                                                    </div>
                                                </td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{category_text(&suggestion.category)}</td>
                                                <td class="px-4 py-3">
                                                    <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", priority_color(&suggestion.priority))}>
                                                        {priority_text(&suggestion.priority)}
                                                    </span>
                                                </td>
                                                <td class="px-4 py-3">
                                                    <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", status_color(&suggestion.status))}>
                                                        {status_text(&suggestion.status)}
                                                    </span>
                                                </td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{suggestion.estimated_impact.clone()}</td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{suggestion.implementation_effort.clone()}</td>
                                                <td class="px-4 py-3">
                                                    <button 
                                                        class="bg-blue-500 hover:bg-blue-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200"
                                                        on:click=move |_| view_suggestion(suggestion_for_view.clone())
                                                    >
                                                        "詳細"
                                                    </button>
                                                </td>
                                            </tr>
                                        }
                                    }
                                />
                            </tbody>
                        </table>
                    </div>
                </div>

                // Mobile Card View
                <div class="lg:hidden space-y-4">
                    <For
                        each=filtered_suggestions
                        key=|suggestion| suggestion.id
                        children=move |suggestion| {
                            let suggestion_for_view = suggestion.clone();
                            view! {
                                <div class="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 p-4">
                                    <div class="flex justify-between items-start mb-3">
                                        <div class="flex-1">
                                            <h3 class="font-semibold text-white mb-1">{suggestion.title.clone()}</h3>
                                            <p class="text-slate-300 text-sm">{category_text(&suggestion.category)}</p>
                                        </div>
                                        <div class="flex flex-col gap-1">
                                            <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", priority_color(&suggestion.priority))}>
                                                {priority_text(&suggestion.priority)}
                                            </span>
                                            <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", status_color(&suggestion.status))}>
                                                {status_text(&suggestion.status)}
                                            </span>
                                        </div>
                                    </div>
                                    
                                    <p class="text-slate-400 text-sm mb-4 line-clamp-3">{suggestion.description.clone()}</p>
                                    
                                    <div class="grid grid-cols-2 gap-2 mb-4 text-sm">
                                        <div>
                                            <span class="text-slate-400">"予想効果: "</span>
                                            <span class="text-slate-300">{suggestion.estimated_impact.clone()}</span>
                                        </div>
                                        <div>
                                            <span class="text-slate-400">"実装期間: "</span>
                                            <span class="text-slate-300">{suggestion.implementation_effort.clone()}</span>
                                        </div>
                                    </div>
                                    
                                    <div class="flex gap-2">
                                        <button 
                                            class="flex-1 bg-blue-500 hover:bg-blue-600 py-2 rounded text-sm font-medium transition-colors duration-200"
                                            on:click=move |_| view_suggestion(suggestion_for_view.clone())
                                        >
                                            "詳細を見る"
                                        </button>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </div>

            // Detail Modal
            <Show when=move || show_detail_modal.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-4xl mx-4 border border-slate-700 max-h-[90vh] overflow-y-auto">
                        {
                            if let Some(suggestion) = viewing_suggestion.get() {
                                view! {
                                    <div>
                                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                                            {suggestion.title.clone()}
                                        </h2>
                                        
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4">"基本情報"</h3>
                                                <div class="space-y-3">
                                                    <div>
                                                        <span class="text-slate-400">"カテゴリ: "</span>
                                                        <span class="text-white">{category_text(&suggestion.category)}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"優先度: "</span>
                                                        <span class={format!("px-3 py-1 rounded-full text-xs font-medium {}", priority_color(&suggestion.priority))}>
                                                            {priority_text(&suggestion.priority)}
                                                        </span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"ステータス: "</span>
                                                        <span class={format!("px-3 py-1 rounded-full text-xs font-medium {}", status_color(&suggestion.status))}>
                                                            {status_text(&suggestion.status)}
                                                        </span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"提案者: "</span>
                                                        <span class="text-white">{suggestion.suggested_by.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"作成日: "</span>
                                                        <span class="text-white">{suggestion.created_date.clone()}</span>
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4">"効果・実装情報"</h3>
                                                <div class="space-y-3">
                                                    <div>
                                                        <span class="text-slate-400">"予想効果: "</span>
                                                        <span class="text-white">{suggestion.estimated_impact.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"実装期間: "</span>
                                                        <span class="text-white">{suggestion.implementation_effort.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"タグ: "</span>
                                                        <div class="flex flex-wrap gap-1 mt-1">
                                                            <For
                                                                each=move || suggestion.tags.clone()
                                                                key=|tag| tag.clone()
                                                                children=move |tag| {
                                                                    view! {
                                                                        <span class="px-2 py-1 bg-slate-700 text-slate-300 rounded text-xs">
                                                                            {tag}
                                                                        </span>
                                                                    }
                                                                }
                                                            />
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="mb-6">
                                            <h3 class="text-lg font-semibold mb-4">"詳細説明"</h3>
                                            <p class="text-slate-300 bg-slate-700/50 p-4 rounded-lg leading-relaxed">
                                                {suggestion.description.clone()}
                                            </p>
                                        </div>
                                        
                                        <div class="flex gap-3">
                                            <button class="bg-green-500 hover:bg-green-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "承認"
                                            </button>
                                            <button class="bg-blue-500 hover:bg-blue-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "検討中にする"
                                            </button>
                                            <button class="bg-red-500 hover:bg-red-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "却下"
                                            </button>
                                            <button 
                                                class="bg-slate-600 hover:bg-slate-500 px-4 py-2 rounded-lg font-medium transition-colors duration-200"
                                                on:click=move |_| {
                                                    set_viewing_suggestion.set(None);
                                                    set_show_detail_modal.set(false);
                                                }
                                            >
                                                "閉じる"
                                            </button>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }
                    </div>
                </div>
            </Show>
        </div>
    }
}