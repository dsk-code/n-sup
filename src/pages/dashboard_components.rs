use leptos::prelude::*;
use crate::pages::dashboard::{FinancialMetrics, ExecutiveKPI, PredictiveInsight, Trend, RiskLevel, InsightCategory};

#[component]
pub fn ExecutiveFinancialCard(metrics: FinancialMetrics) -> impl IntoView {
    view! {
        <div class="bg-gradient-to-br from-slate-800/80 to-slate-900/80 backdrop-blur-lg rounded-2xl p-8 border border-yellow-500/30 shadow-2xl">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                // Total Cost Savings
                <div class="text-center">
                    <div class="text-4xl font-bold text-green-400 mb-2">
                        "¥" {format!("{:.1}億", metrics.total_cost_savings as f32 / 100000000.0)}
                    </div>
                    <div class="text-sm text-slate-300 mb-3">"年間コスト削減効果"</div>
                    <div class="w-full bg-slate-700 rounded-full h-3">
                        <div class="bg-gradient-to-r from-green-500 to-emerald-400 h-3 rounded-full animate-pulse" style="width: 95%"></div>
                    </div>
                    <div class="text-xs text-slate-400 mt-2">"目標比 +15.2%"</div>
                </div>

                // Productivity Value
                <div class="text-center">
                    <div class="text-4xl font-bold text-blue-400 mb-2">
                        "¥" {format!("{:.1}億", metrics.productivity_gain_value as f32 / 100000000.0)}
                    </div>
                    <div class="text-sm text-slate-300 mb-3">"生産性向上価値"</div>
                    <div class="w-full bg-slate-700 rounded-full h-3">
                        <div class="bg-gradient-to-r from-blue-500 to-cyan-400 h-3 rounded-full animate-pulse" style="width: 88%"></div>
                    </div>
                    <div class="text-xs text-slate-400 mt-2">"ROI: +32.4%"</div>
                </div>

                // Equipment Utilization
                <div class="text-center">
                    <div class="text-4xl font-bold text-purple-400 mb-2">
                        {format!("{:.1}%", metrics.equipment_utilization)}
                    </div>
                    <div class="text-sm text-slate-300 mb-3">"設備稼働率"</div>
                    <div class="w-full bg-slate-700 rounded-full h-3">
                        <div 
                            class="bg-gradient-to-r from-purple-500 to-pink-400 h-3 rounded-full animate-pulse"
                            style=format!("width: {}%", metrics.equipment_utilization)
                        ></div>
                    </div>
                    <div class="text-xs text-slate-400 mt-2">"業界平均比 +12.7%"</div>
                </div>
            </div>

            <div class="mt-8 pt-6 border-t border-slate-700/50">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6 text-center">
                    <div>
                        <div class="text-2xl font-bold text-orange-400">{format!("{}%", metrics.energy_cost_reduction)}</div>
                        <div class="text-xs text-slate-400">"エネルギーコスト削減"</div>
                    </div>
                    <div>
                        <div class="text-2xl font-bold text-cyan-400">
                            "¥" {format!("{:.1}億", metrics.quality_improvement_value as f32 / 100000000.0)}
                        </div>
                        <div class="text-xs text-slate-400">"品質改善価値"</div>
                    </div>
                    <div>
                        <div class="text-2xl font-bold text-yellow-400">{format!("{}%", metrics.maintenance_cost_reduction)}</div>
                        <div class="text-xs text-slate-400">"メンテナンスコスト削減"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ExecutiveKPIGrid(kpis: Vec<ExecutiveKPI>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <For
                each=move || kpis.clone()
                key=|kpi| kpi.name.clone()
                children=move |kpi| {
                    let trend_icon = match kpi.trend {
                        Trend::Up => "📈",
                        Trend::Down => "📉",
                        Trend::Stable => "➡️",
                    };
                    let trend_color = match kpi.trend {
                        Trend::Up => "text-green-400",
                        Trend::Down => "text-red-400",
                        Trend::Stable => "text-yellow-400",
                    };
                    
                    view! {
                        <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border border-slate-700 hover:border-blue-500/50 transition-all duration-300">
                            <div class="flex justify-between items-start mb-4">
                                <div>
                                    <h3 class="text-lg font-semibold text-white mb-1">{kpi.name.clone()}</h3>
                                    <div class="text-sm text-slate-400">{kpi.department.clone()}</div>
                                </div>
                                <div class="text-right">
                                    <div class={format!("text-2xl {}", trend_color)}>{trend_icon}</div>
                                    <div class="text-xs text-slate-400">"Impact: " {format!("{:.1}", kpi.impact_score)}</div>
                                </div>
                            </div>
                            
                            <div class="grid grid-cols-2 gap-4 mb-4">
                                <div>
                                    <div class="text-2xl font-bold text-white">{format!("{:.1}", kpi.current_value)}</div>
                                    <div class="text-xs text-slate-400">"現在値"</div>
                                </div>
                                <div>
                                    <div class="text-lg text-slate-300">{format!("{:.1}", kpi.target_value)}</div>
                                    <div class="text-xs text-slate-400">"目標値"</div>
                                </div>
                            </div>
                            
                            <div class="mb-4">
                                <div class="flex justify-between text-xs text-slate-400 mb-1">
                                    <span>"進捗"</span>
                                    <span>{format!("{:.1}%", (kpi.current_value / kpi.target_value * 100.0).min(100.0))}</span>
                                </div>
                                <div class="w-full bg-slate-700 rounded-full h-2">
                                    <div 
                                        class="bg-gradient-to-r from-blue-500 to-purple-500 h-2 rounded-full transition-all duration-1000"
                                        style=format!("width: {}%", (kpi.current_value / kpi.target_value * 100.0).min(100.0))
                                    ></div>
                                </div>
                            </div>
                            
                            <div class="grid grid-cols-2 gap-4 text-center text-sm">
                                <div>
                                    <div class="text-green-400 font-semibold">{format!("{:.1}", kpi.forecast_3_months)}</div>
                                    <div class="text-xs text-slate-400">"3ヶ月予測"</div>
                                </div>
                                <div>
                                    <div class="text-yellow-400 font-semibold">{format!("+{:.1}%", kpi.roi_percentage)}</div>
                                    <div class="text-xs text-slate-400">"ROI"</div>
                                </div>
                            </div>
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn PredictiveInsightsCard(insights: Vec<PredictiveInsight>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <For
                each=move || insights.clone()
                key=|insight| insight.title.clone()
                children=move |insight| {
                    let risk_color = match insight.risk_level {
                        RiskLevel::Low => "border-green-500/30 bg-green-900/20",
                        RiskLevel::Medium => "border-yellow-500/30 bg-yellow-900/20",
                        RiskLevel::High => "border-red-500/30 bg-red-900/20",
                    };
                    let risk_text = match insight.risk_level {
                        RiskLevel::Low => "低リスク",
                        RiskLevel::Medium => "中リスク", 
                        RiskLevel::High => "高リスク",
                    };
                    let category_icon = match insight.category {
                        InsightCategory::CostOptimization => "💰",
                        InsightCategory::ProductivityGain => "🚀",
                        InsightCategory::QualityImprovement => "⭐",
                        InsightCategory::PredictiveMaintenance => "🔧",
                        InsightCategory::SafetyEnhancement => "🛡️",
                    };
                    
                    view! {
                        <div class={format!("bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border {} transition-all duration-300 hover:scale-105", risk_color)}>
                            <div class="flex items-start justify-between mb-4">
                                <div class="text-3xl">{category_icon}</div>
                                <div class="text-right">
                                    <div class="text-sm text-slate-400">"信頼度"</div>
                                    <div class="text-xl font-bold text-green-400">{format!("{:.1}%", insight.confidence)}</div>
                                </div>
                            </div>
                            
                            <h3 class="text-lg font-semibold text-white mb-3">{insight.title.clone()}</h3>
                            <p class="text-sm text-slate-300 mb-4 leading-relaxed">{insight.description.clone()}</p>
                            
                            <div class="space-y-3">
                                <div class="flex justify-between">
                                    <span class="text-slate-400 text-sm">"削減効果:"</span>
                                    <span class="text-green-400 font-semibold">
                                        "¥" {format!("{:.1}M", insight.potential_savings as f32 / 1000000.0)}
                                    </span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-slate-400 text-sm">"実装期間:"</span>
                                    <span class="text-white">{insight.implementation_effort.clone()}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-slate-400 text-sm">"リスク:"</span>
                                    <span class="text-white">{risk_text}</span>
                                </div>
                            </div>
                            
                            <button class="w-full mt-4 bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 py-2 rounded-lg font-medium transition-all duration-300">
                                "詳細分析"
                            </button>
                        </div>
                    }
                }
            />
        </div>
    }
}