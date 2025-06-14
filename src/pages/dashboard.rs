use leptos::prelude::*;
use leptos_router::components::A;

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

    let alerts = vec![
        AlertItem {
            id: 1,
            alert_type: AlertType::Warning,
            message: "工具 DR-005 の交換時期が近づいています".to_string(),
            timestamp: "今日".to_string(),
        },
        AlertItem {
            id: 2,
            alert_type: AlertType::Info,
            message: "新しいAI最適化提案が3件あります".to_string(),
            timestamp: "今日".to_string(),
        },
        AlertItem {
            id: 3,
            alert_type: AlertType::Success,
            message: "今月の生産効率目標を達成しました".to_string(),
            timestamp: "昨日".to_string(),
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