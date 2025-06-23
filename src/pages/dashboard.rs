use leptos::prelude::*;
use leptos_router::components::A;
use wasm_bindgen::prelude::*;

use crate::pages::dashboard_components::{ExecutiveFinancialCard, ExecutiveKPIGrid, PredictiveInsightsCard};

#[derive(Clone, Debug)]
pub struct DashboardStats {
    pub tools_total: u32,
    pub tools_available: u32,
    pub employees_active: u32,
    pub nc_programs: u32,
    pub active_projects: u32,
    pub completion_rate: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecentActivity {
    pub id: u32,
    pub activity_type: String,
    pub description: String,
    pub timestamp: String,
    pub user: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlertItem {
    pub id: u32,
    pub alert_type: AlertType,
    pub message: String,
    pub timestamp: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AlertType {
    Warning,
    Info,
    Success,
    Critical,
}

#[derive(Clone, Debug)]
pub struct ProductionMetrics {
    pub current_efficiency: f32,
    pub target_efficiency: f32,
    pub production_count: u32,
    pub target_count: u32,
    pub quality_rate: f32,
    pub downtime_minutes: u32,
}

#[derive(Clone, Debug)]
pub struct WeeklyData {
    pub day: String,
    pub efficiency: f32,
    pub production: u32,
    pub quality: f32,
}

#[derive(Clone, Debug)]
pub struct ExecutiveKPI {
    pub name: String,
    pub current_value: f32,
    pub target_value: f32,
    pub trend: Trend,
    pub impact_score: f32,
    pub department: String,
    pub forecast_3_months: f32,
    pub roi_percentage: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Trend {
    Up,
    Down,
    Stable,
}

#[derive(Clone, Debug)]
pub struct PredictiveInsight {
    pub title: String,
    pub description: String,
    pub confidence: f32,
    pub potential_savings: u32,
    pub implementation_effort: String,
    pub risk_level: RiskLevel,
    pub category: InsightCategory,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InsightCategory {
    CostOptimization,
    ProductivityGain,
    QualityImprovement,
    PredictiveMaintenance,
    SafetyEnhancement,
}

#[derive(Clone, Debug)]
pub struct FinancialMetrics {
    pub total_cost_savings: u32,
    pub productivity_gain_value: u32,
    pub equipment_utilization: f32,
    pub energy_cost_reduction: f32,
    pub quality_improvement_value: u32,
    pub maintenance_cost_reduction: f32,
}

#[component]
pub fn Dashboard() -> impl IntoView {
    // Sample dashboard data
    let stats = DashboardStats {
        tools_total: 156,
        tools_available: 134,
        employees_active: 42,
        nc_programs: 89,
        active_projects: 12,
        completion_rate: 94.5,
    };

    let recent_activities = vec![
        RecentActivity {
            id: 1,
            activity_type: "Tool".to_string(),
            description: "CNC旋盤 TL-001 がメンテナンス完了".to_string(),
            timestamp: "2分前".to_string(),
            user: "田中工程管理者".to_string(),
        },
        RecentActivity {
            id: 2,
            activity_type: "Program".to_string(),
            description: "NCプログラム NC-2024-003 が最適化されました".to_string(),
            timestamp: "15分前".to_string(),
            user: "AI最適化システム".to_string(),
        },
        RecentActivity {
            id: 3,
            activity_type: "Employee".to_string(),
            description: "佐藤操作員 が新しいプロジェクトに参加".to_string(),
            timestamp: "1時間前".to_string(),
            user: "人事管理システム".to_string(),
        },
    ];

    let production_metrics = ProductionMetrics {
        current_efficiency: 87.5,
        target_efficiency: 85.0,
        production_count: 1247,
        target_count: 1200,
        quality_rate: 98.3,
        downtime_minutes: 45,
    };

    let weekly_data = vec![
        WeeklyData { day: "月".to_string(), efficiency: 82.5, production: 220, quality: 97.8 },
        WeeklyData { day: "火".to_string(), efficiency: 85.1, production: 235, quality: 98.1 },
        WeeklyData { day: "水".to_string(), efficiency: 88.3, production: 250, quality: 98.5 },
        WeeklyData { day: "木".to_string(), efficiency: 90.2, production: 265, quality: 98.9 },
        WeeklyData { day: "金".to_string(), efficiency: 87.5, production: 247, quality: 98.3 },
        WeeklyData { day: "土".to_string(), efficiency: 79.8, production: 180, quality: 97.5 },
        WeeklyData { day: "日".to_string(), efficiency: 75.2, production: 150, quality: 97.2 },
    ];

    let executive_kpis = vec![
        ExecutiveKPI {
            name: "総合設備効率 (OEE)".to_string(),
            current_value: 87.3,
            target_value: 85.0,
            trend: Trend::Up,
            impact_score: 9.2,
            department: "製造部門".to_string(),
            forecast_3_months: 91.5,
            roi_percentage: 23.7,
        },
        ExecutiveKPI {
            name: "品質コスト削減".to_string(),
            current_value: 15.2,
            target_value: 12.0,
            trend: Trend::Down,
            impact_score: 8.8,
            department: "品質管理部門".to_string(),
            forecast_3_months: 9.1,
            roi_percentage: 31.4,
        },
        ExecutiveKPI {
            name: "予測保全効果".to_string(),
            current_value: 94.1,
            target_value: 90.0,
            trend: Trend::Up,
            impact_score: 9.6,
            department: "保全部門".to_string(),
            forecast_3_months: 97.2,
            roi_percentage: 42.8,
        },
        ExecutiveKPI {
            name: "エネルギー効率".to_string(),
            current_value: 78.5,
            target_value: 80.0,
            trend: Trend::Up,
            impact_score: 7.9,
            department: "環境・省エネ部門".to_string(),
            forecast_3_months: 82.3,
            roi_percentage: 18.6,
        },
    ];

    let predictive_insights = vec![
        PredictiveInsight {
            title: "機械学習による需要予測最適化".to_string(),
            description: "過去3年のデータ分析により、来月の需要を97.3%の精度で予測。在庫コストを24%削減可能".to_string(),
            confidence: 97.3,
            potential_savings: 2840000,
            implementation_effort: "2週間".to_string(),
            risk_level: RiskLevel::Low,
            category: InsightCategory::CostOptimization,
        },
        PredictiveInsight {
            title: "AIベース異常検知システム".to_string(),
            description: "センサーデータからの早期異常検知により、計画外停止を78%削減。年間1.2億円の損失回避".to_string(),
            confidence: 94.7,
            potential_savings: 120000000,
            implementation_effort: "1ヶ月".to_string(),
            risk_level: RiskLevel::Medium,
            category: InsightCategory::PredictiveMaintenance,
        },
        PredictiveInsight {
            title: "工程間最適化アルゴリズム".to_string(),
            description: "リアルタイム工程調整により生産性32%向上。スループット時間を平均2.1時間短縮".to_string(),
            confidence: 89.4,
            potential_savings: 45600000,
            implementation_effort: "3週間".to_string(),
            risk_level: RiskLevel::Low,
            category: InsightCategory::ProductivityGain,
        },
    ];

    let financial_metrics = FinancialMetrics {
        total_cost_savings: 285400000,
        productivity_gain_value: 156800000,
        equipment_utilization: 92.7,
        energy_cost_reduction: 18.4,
        quality_improvement_value: 67200000,
        maintenance_cost_reduction: 23.1,
    };

    let alerts = vec![
        AlertItem {
            id: 1,
            alert_type: AlertType::Critical,
            message: "AI検知: ライン3で品質低下リスク増加 (信頼度: 94.2%)".to_string(),
            timestamp: "5分前".to_string(),
        },
        AlertItem {
            id: 2,
            alert_type: AlertType::Warning,
            message: "予測保全: 機械MC-007のベアリング交換を2日以内に推奨".to_string(),
            timestamp: "30分前".to_string(),
        },
        AlertItem {
            id: 3,
            alert_type: AlertType::Info,
            message: "ROI分析完了: 新AI提案で年間2.84億円のコスト削減見込み".to_string(),
            timestamp: "1時間前".to_string(),
        },
        AlertItem {
            id: 4,
            alert_type: AlertType::Success,
            message: "OEE目標達成: Q2の総合設備効率が87.3%に到達".to_string(),
            timestamp: "2時間前".to_string(),
        },
    ];

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
            // Header
            <header class="bg-slate-800/50 backdrop-blur-lg border-b border-slate-700">
                <div class="max-w-7xl mx-auto px-6 py-4">
                    <div class="flex items-center justify-between">
                        <div class="flex items-center space-x-4">
                            <div class="text-2xl font-bold bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
                                "N-Sup"
                            </div>
                            <span class="text-slate-400">"/"</span>
                            <h1 class="text-xl text-white font-semibold">"ダッシュボード"</h1>
                        </div>
                        <div class="flex items-center space-x-4">
                            <div class="text-sm text-slate-300">"ようこそ、管理者さん"</div>
                            <div class="w-8 h-8 bg-gradient-to-r from-blue-500 to-purple-500 rounded-full flex items-center justify-center text-white text-sm font-semibold">
                                "管"
                            </div>
                        </div>
                    </div>
                </div>
            </header>

            <div class="max-w-7xl mx-auto px-6 py-8">
                // Executive Financial Overview
                <div class="mb-8">
                    <h2 class="text-2xl font-bold text-white mb-6 flex items-center gap-3">
                        <span class="text-3xl">"💼"</span>
                        "Executive Financial Dashboard"
                        <span class="text-sm bg-gradient-to-r from-gold-400 to-yellow-500 px-3 py-1 rounded-full text-black font-semibold">"PRESIDENT VIEW"</span>
                    </h2>
                    <ExecutiveFinancialCard metrics=financial_metrics.clone() />
                </div>

                // Executive KPIs
                <div class="mb-8">
                    <h2 class="text-xl font-semibold text-white mb-6 flex items-center gap-2">
                        <span class="text-2xl">"📊"</span>
                        "Strategic KPI Dashboard"
                    </h2>
                    <ExecutiveKPIGrid kpis=executive_kpis.clone() />
                </div>

                // AI Predictive Insights
                <div class="mb-8">
                    <h2 class="text-xl font-semibold text-white mb-6 flex items-center gap-2">
                        <span class="text-2xl">"🤖"</span>
                        "AI-Powered Predictive Insights"
                    </h2>
                    <PredictiveInsightsCard insights=predictive_insights.clone() />
                </div>

                // Stats Grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-6 gap-6 mb-8">
                    <StatsCard 
                        title="総工具数"
                        value=stats.tools_total.to_string()
                        color="blue"
                        icon="🔧"
                    />
                    <StatsCard 
                        title="利用可能"
                        value=stats.tools_available.to_string()
                        color="green"
                        icon="✅"
                    />
                    <StatsCard 
                        title="稼働中従業員"
                        value=stats.employees_active.to_string()
                        color="purple"
                        icon="👥"
                    />
                    <StatsCard 
                        title="NCプログラム"
                        value=stats.nc_programs.to_string()
                        color="cyan"
                        icon="⚙️"
                    />
                    <StatsCard 
                        title="進行中プロジェクト"
                        value=stats.active_projects.to_string()
                        color="orange"
                        icon="📋"
                    />
                    <StatsCard 
                        title="完了率"
                        value=format!("{}%", stats.completion_rate)
                        color="green"
                        icon="📊"
                    />
                </div>

                // Production Metrics Section
                <div class="mb-8">
                    <h2 class="text-xl font-semibold text-white mb-6">"生産メトリクス - リアルタイム"</h2>
                    <ProductionMetricsCard metrics=production_metrics.clone() />
                </div>

                // Weekly Performance Chart
                <div class="mb-8">
                    <h2 class="text-xl font-semibold text-white mb-6">"週間パフォーマンス"</h2>
                    <WeeklyPerformanceChart data=weekly_data.clone() />
                </div>

                // Main Content Grid
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    // Quick Actions
                    <div class="lg:col-span-1">
                        <QuickActions />
                    </div>

                    // Recent Activities
                    <div class="lg:col-span-1">
                        <RecentActivitiesCard activities=recent_activities />
                    </div>

                    // Alerts and Notifications
                    <div class="lg:col-span-1">
                        <AlertsCard alerts=alerts />
                    </div>
                </div>

                // Navigation Grid
                <div class="mt-8">
                    <h2 class="text-xl font-semibold text-white mb-6">"システムナビゲーション"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                        <NavigationCard 
                            title="工具管理"
                            description="工具の在庫管理とメンテナンス"
                            href="/n-sup/tools"
                            icon="🔧"
                            color="blue"
                        />
                        <NavigationCard 
                            title="従業員管理"
                            description="従業員情報と勤務状況"
                            href="/n-sup/employees"
                            icon="👥"
                            color="purple"
                        />
                        <NavigationCard 
                            title="NCプログラム"
                            description="プログラム管理とバージョン管理"
                            href="/n-sup/nc-programs"
                            icon="⚙️"
                            color="cyan"
                        />
                        <NavigationCard 
                            title="AI支援"
                            description="AI最適化とプログラム生成"
                            href="/n-sup/nc-support"
                            icon="🤖"
                            color="green"
                        />
                        <NavigationCard 
                            title="AI工具提案"
                            description="機械学習による改善提案"
                            href="/n-sup/ai-suggestions"
                            icon="💡"
                            color="yellow"
                        />
                        <NavigationCard 
                            title="チャット"
                            description="チーム内コミュニケーション"
                            href="/n-sup/chat"
                            icon="💬"
                            color="orange"
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StatsCard(
    title: &'static str,
    value: String,
    color: &'static str,
    icon: &'static str,
) -> impl IntoView {
    let color_classes = match color {
        "blue" => "from-blue-500 to-blue-600",
        "green" => "from-green-500 to-green-600",
        "purple" => "from-purple-500 to-purple-600",
        "cyan" => "from-cyan-500 to-cyan-600",
        "orange" => "from-orange-500 to-orange-600",
        "yellow" => "from-yellow-500 to-yellow-600",
        _ => "from-gray-500 to-gray-600",
    };

    view! {
        <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border border-slate-700 hover:border-slate-600 transition-all duration-300">
            <div class="flex items-center justify-between mb-4">
                <div class=format!("text-2xl bg-gradient-to-r {} bg-clip-text text-transparent", color_classes)>
                    {icon}
                </div>
            </div>
            <div class="text-2xl font-bold text-white mb-1">{value}</div>
            <div class="text-sm text-slate-400">{title}</div>
        </div>
    }
}

#[component]
fn QuickActions() -> impl IntoView {
    view! {
        <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border border-slate-700">
            <h3 class="text-lg font-semibold text-white mb-4">"クイックアクション"</h3>
            <div class="space-y-3">
                <A href="/n-sup/tools" attr:class="block w-full bg-gradient-to-r from-blue-500 to-blue-600 text-white py-3 px-4 rounded-lg hover:from-blue-600 hover:to-blue-700 transition-all duration-300 text-center font-medium">
                    "新しい工具を追加"
                </A>
                <A href="/n-sup/employees" attr:class="block w-full bg-gradient-to-r from-purple-500 to-purple-600 text-white py-3 px-4 rounded-lg hover:from-purple-600 hover:to-purple-700 transition-all duration-300 text-center font-medium">
                    "従業員を登録"
                </A>
                <A href="/n-sup/nc-programs" attr:class="block w-full bg-gradient-to-r from-cyan-500 to-cyan-600 text-white py-3 px-4 rounded-lg hover:from-cyan-600 hover:to-cyan-700 transition-all duration-300 text-center font-medium">
                    "NCプログラム作成"
                </A>
                <A href="/n-sup/chat" attr:class="block w-full bg-gradient-to-r from-orange-500 to-orange-600 text-white py-3 px-4 rounded-lg hover:from-orange-600 hover:to-orange-700 transition-all duration-300 text-center font-medium">
                    "チーム会議を開始"
                </A>
            </div>
        </div>
    }
}

#[component]
fn RecentActivitiesCard(activities: Vec<RecentActivity>) -> impl IntoView {
    view! {
        <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border border-slate-700">
            <h3 class="text-lg font-semibold text-white mb-4">"最近のアクティビティ"</h3>
            <div class="space-y-4">
                {activities.into_iter().map(|activity| {
                    view! {
                        <div class="flex items-start space-x-3 p-3 bg-slate-700/30 rounded-lg">
                            <div class="w-2 h-2 bg-blue-400 rounded-full mt-2 flex-shrink-0"></div>
                            <div class="flex-1">
                                <div class="text-sm text-white">{activity.description}</div>
                                <div class="text-xs text-slate-400 mt-1">
                                    {activity.user} " • " {activity.timestamp}
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            <A href="/n-sup/activities" attr:class="block text-center text-sm text-blue-400 hover:text-blue-300 mt-4">
                "すべてのアクティビティを表示"
            </A>
        </div>
    }
}

#[component]
fn AlertsCard(alerts: Vec<AlertItem>) -> impl IntoView {
    view! {
        <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border border-slate-700">
            <h3 class="text-lg font-semibold text-white mb-4">"通知とアラート"</h3>
            <div class="space-y-3">
                {alerts.into_iter().map(|alert| {
                    let (icon, color) = match alert.alert_type {
                        AlertType::Critical => ("🚨", "text-red-400"),
                        AlertType::Warning => ("⚠️", "text-yellow-400"),
                        AlertType::Info => ("ℹ️", "text-blue-400"),
                        AlertType::Success => ("✅", "text-green-400"),
                    };
                    
                    view! {
                        <div class="flex items-start space-x-3 p-3 bg-slate-700/30 rounded-lg">
                            <div class=format!("text-sm {}", color)>{icon}</div>
                            <div class="flex-1">
                                <div class="text-sm text-white">{alert.message}</div>
                                <div class="text-xs text-slate-400 mt-1">{alert.timestamp}</div>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn NavigationCard(
    title: &'static str,
    description: &'static str,
    href: &'static str,
    icon: &'static str,
    color: &'static str,
) -> impl IntoView {
    let hover_classes = match color {
        "blue" => "hover:border-blue-500",
        "green" => "hover:border-green-500",
        "purple" => "hover:border-purple-500",
        "cyan" => "hover:border-cyan-500",
        "orange" => "hover:border-orange-500",
        "yellow" => "hover:border-yellow-500",
        _ => "hover:border-gray-500",
    };

    view! {
        <A href=href attr:class=format!("block bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border border-slate-700 {} transition-all duration-300 hover:bg-slate-700/50", hover_classes)>
            <div class="text-3xl mb-3">{icon}</div>
            <h3 class="text-lg font-semibold text-white mb-2">{title}</h3>
            <p class="text-sm text-slate-400">{description}</p>
        </A>
    }
}

#[component]
fn ProductionMetricsCard(metrics: ProductionMetrics) -> impl IntoView {
    let efficiency_percentage = (metrics.current_efficiency / metrics.target_efficiency * 100.0).min(100.0);
    let production_percentage = (metrics.production_count as f32 / metrics.target_count as f32 * 100.0).min(100.0);
    
    view! {
        <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border border-slate-700">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                // Current Efficiency
                <div class="text-center">
                    <div class="text-3xl font-bold text-green-400 mb-2">{format!("{:.1}%", metrics.current_efficiency)}</div>
                    <div class="text-sm text-slate-400 mb-3">"現在の効率"</div>
                    <div class="w-full bg-slate-700 rounded-full h-2">
                        <div 
                            class="bg-gradient-to-r from-green-500 to-green-400 h-2 rounded-full transition-all duration-1000"
                            style=format!("width: {}%", efficiency_percentage)
                        ></div>
                    </div>
                    <div class="text-xs text-slate-500 mt-1">{format!("目標: {:.1}%", metrics.target_efficiency)}</div>
                </div>

                // Production Count
                <div class="text-center">
                    <div class="text-3xl font-bold text-blue-400 mb-2">{metrics.production_count}</div>
                    <div class="text-sm text-slate-400 mb-3">"生産数"</div>
                    <div class="w-full bg-slate-700 rounded-full h-2">
                        <div 
                            class="bg-gradient-to-r from-blue-500 to-blue-400 h-2 rounded-full transition-all duration-1000"
                            style=format!("width: {}%", production_percentage)
                        ></div>
                    </div>
                    <div class="text-xs text-slate-500 mt-1">{format!("目標: {}", metrics.target_count)}</div>
                </div>

                // Quality Rate
                <div class="text-center">
                    <div class="text-3xl font-bold text-purple-400 mb-2">{format!("{:.1}%", metrics.quality_rate)}</div>
                    <div class="text-sm text-slate-400 mb-3">"品質率"</div>
                    <div class="w-full bg-slate-700 rounded-full h-2">
                        <div 
                            class="bg-gradient-to-r from-purple-500 to-purple-400 h-2 rounded-full transition-all duration-1000"
                            style=format!("width: {}%", metrics.quality_rate)
                        ></div>
                    </div>
                    <div class="text-xs text-slate-500 mt-1">"目標: 98.0%"</div>
                </div>

                // Downtime
                <div class="text-center">
                    <div class="text-3xl font-bold text-orange-400 mb-2">{metrics.downtime_minutes}"分"</div>
                    <div class="text-sm text-slate-400 mb-3">"ダウンタイム"</div>
                    <div class="w-full bg-slate-700 rounded-full h-2">
                        <div 
                            class="bg-gradient-to-r from-orange-500 to-red-400 h-2 rounded-full transition-all duration-1000"
                            style=format!("width: {}%", (metrics.downtime_minutes as f32 / 120.0 * 100.0).min(100.0))
                        ></div>
                    </div>
                    <div class="text-xs text-slate-500 mt-1">"目標: < 60分"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn WeeklyPerformanceChart(data: Vec<WeeklyData>) -> impl IntoView {
    let max_efficiency = data.iter().map(|d| d.efficiency).fold(0.0, f32::max);
    let max_production = data.iter().map(|d| d.production).fold(0, u32::max);
    
    view! {
        <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border border-slate-700">
            <div class="mb-6">
                <div class="flex justify-between items-center mb-4">
                    <h3 class="text-lg font-semibold text-white">"週間パフォーマンストレンド"</h3>
                    <div class="flex gap-4 text-sm">
                        <div class="flex items-center gap-2">
                            <div class="w-3 h-3 bg-blue-500 rounded"></div>
                            <span class="text-slate-300">"効率 (%)"</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <div class="w-3 h-3 bg-green-500 rounded"></div>
                            <span class="text-slate-300">"生産数"</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <div class="w-3 h-3 bg-purple-500 rounded"></div>
                            <span class="text-slate-300">"品質 (%)"</span>
                        </div>
                    </div>
                </div>
                
                <div class="grid grid-cols-7 gap-2 h-64">
                    <For
                        each=move || data.clone()
                        key=|day| day.day.clone()
                        children=move |day_data| {
                            let efficiency_height = (day_data.efficiency / max_efficiency * 100.0) as u32;
                            let production_height = (day_data.production as f32 / max_production as f32 * 100.0) as u32;
                            let quality_height = day_data.quality as u32;
                            
                            view! {
                                <div class="flex flex-col items-center">
                                    <div class="flex-1 flex flex-col justify-end w-full space-y-1">
                                        // Efficiency Bar
                                        <div 
                                            class="bg-gradient-to-t from-blue-600 to-blue-400 rounded-sm transition-all duration-1000 hover:from-blue-500 hover:to-blue-300"
                                            style=format!("height: {}%", efficiency_height.min(80))
                                            title=format!("効率: {:.1}%", day_data.efficiency)
                                        ></div>
                                        // Production Bar  
                                        <div 
                                            class="bg-gradient-to-t from-green-600 to-green-400 rounded-sm transition-all duration-1000 hover:from-green-500 hover:to-green-300"
                                            style=format!("height: {}%", production_height.min(80))
                                            title=format!("生産数: {}", day_data.production)
                                        ></div>
                                        // Quality Bar
                                        <div 
                                            class="bg-gradient-to-t from-purple-600 to-purple-400 rounded-sm transition-all duration-1000 hover:from-purple-500 hover:to-purple-300"
                                            style=format!("height: {}%", quality_height.min(80))
                                            title=format!("品質: {:.1}%", day_data.quality)
                                        ></div>
                                    </div>
                                    <div class="text-xs text-slate-400 mt-2 font-medium">{day_data.day}</div>
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}