use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct NcOptimization {
    pub id: u32,
    pub program_name: String,
    pub original_code: String,
    pub optimized_code: String,
    pub optimization_type: OptimizationType,
    pub improvement_percentage: f32,
    pub execution_time_reduction: String,
    pub tool_life_improvement: String,
    pub surface_quality_improvement: String,
    pub suggested_by: String,
    pub created_date: String,
    pub status: OptimizationStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OptimizationType {
    SpeedOptimization,
    ToolLifeExtension,
    SurfaceQualityImprovement,
    CuttingPathOptimization,
    CoolantOptimization,
    PowerConsumptionReduction,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OptimizationStatus {
    New,
    Analyzing,
    Ready,
    Applied,
    Testing,
    Verified,
}

#[derive(Clone, Debug)]
pub struct CodeTemplate {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub machine_type: String,
    pub operation_type: String,
    pub template_code: String,
    pub parameters: Vec<String>,
    pub usage_count: u32,
    pub last_used: String,
}

#[component]
pub fn NcProgramSupport() -> impl IntoView {
    let (optimizations, _set_optimizations) = signal(vec![
        NcOptimization {
            id: 1,
            program_name: "アルミブラケット加工 O1001".to_string(),
            original_code: "G01 F200 X10.0 Y10.0\nG01 F150 X20.0\nG01 F200 Y20.0\nG00 Z5.0".to_string(),
            optimized_code: "G01 F300 X10.0 Y10.0\nG01 F250 X20.0\nG01 F300 Y20.0\nG00 Z5.0".to_string(),
            optimization_type: OptimizationType::SpeedOptimization,
            improvement_percentage: 25.0,
            execution_time_reduction: "1分30秒短縮".to_string(),
            tool_life_improvement: "15%向上".to_string(),
            surface_quality_improvement: "Ra値10%改善".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-12".to_string(),
            status: OptimizationStatus::Ready,
        },
        NcOptimization {
            id: 2,
            program_name: "ステンレスパイプ O2003".to_string(),
            original_code: "G01 F100 X15.0\nM08\nG01 F80 Z-2.0\nG01 F100 X25.0\nM09".to_string(),
            optimized_code: "G01 F120 X15.0\nM08\nG01 F100 Z-2.0\nG01 F130 X25.0\nM09".to_string(),
            optimization_type: OptimizationType::ToolLifeExtension,
            improvement_percentage: 18.0,
            execution_time_reduction: "45秒短縮".to_string(),
            tool_life_improvement: "25%向上".to_string(),
            surface_quality_improvement: "表面粗さ改善".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-11".to_string(),
            status: OptimizationStatus::Analyzing,
        },
        NcOptimization {
            id: 3,
            program_name: "鉄鋼プレート穴あけ O3005".to_string(),
            original_code: "G81 X10.0 Y10.0 Z-5.0 R2.0 F50\nG80\nG00 X20.0 Y10.0\nG81 Z-5.0 R2.0 F50".to_string(),
            optimized_code: "G81 X10.0 Y10.0 Z-5.0 R2.0 F75\nX20.0\nG80".to_string(),
            optimization_type: OptimizationType::CuttingPathOptimization,
            improvement_percentage: 35.0,
            execution_time_reduction: "2分短縮".to_string(),
            tool_life_improvement: "20%向上".to_string(),
            surface_quality_improvement: "バリ除去効果".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-10".to_string(),
            status: OptimizationStatus::Verified,
        },
        NcOptimization {
            id: 4,
            program_name: "表面品質改善 O4001".to_string(),
            original_code: "G01 F150 X30.0\nG01 F120 Y30.0".to_string(),
            optimized_code: "G01 F180 X30.0\nG01 F160 Y30.0".to_string(),
            optimization_type: OptimizationType::SurfaceQualityImprovement,
            improvement_percentage: 20.0,
            execution_time_reduction: "30秒短縮".to_string(),
            tool_life_improvement: "12%向上".to_string(),
            surface_quality_improvement: "Ra値15%改善".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-09".to_string(),
            status: OptimizationStatus::New,
        },
        NcOptimization {
            id: 5,
            program_name: "切削液最適化 O5001".to_string(),
            original_code: "M08\nG01 F100 X25.0\nM09".to_string(),
            optimized_code: "M08\nG04 P0.5\nG01 F120 X25.0\nG04 P0.2\nM09".to_string(),
            optimization_type: OptimizationType::CoolantOptimization,
            improvement_percentage: 15.0,
            execution_time_reduction: "20秒短縮".to_string(),
            tool_life_improvement: "30%向上".to_string(),
            surface_quality_improvement: "切削熱減少".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-08".to_string(),
            status: OptimizationStatus::Applied,
        },
        NcOptimization {
            id: 6,
            program_name: "省電力加工 O6001".to_string(),
            original_code: "S3000 M03\nG01 F200 X40.0".to_string(),
            optimized_code: "S2500 M03\nG01 F180 X40.0".to_string(),
            optimization_type: OptimizationType::PowerConsumptionReduction,
            improvement_percentage: 12.0,
            execution_time_reduction: "10秒短縮".to_string(),
            tool_life_improvement: "8%向上".to_string(),
            surface_quality_improvement: "振動減少".to_string(),
            suggested_by: "AIシステム".to_string(),
            created_date: "2024-06-07".to_string(),
            status: OptimizationStatus::Testing,
        },
    ]);

    let (templates, _set_templates) = signal(vec![
        CodeTemplate {
            id: 1,
            name: "アルミ材フライス加工テンプレート".to_string(),
            description: "アルミニウム合金の標準フライス加工用テンプレート".to_string(),
            machine_type: "マシニングセンタ".to_string(),
            operation_type: "フライス加工".to_string(),
            template_code: "G90 G54 G00 X0 Y0\nS{SPINDLE_SPEED} M03\nG43 H{TOOL_NUMBER} Z100\nM08\nG01 Z{START_DEPTH} F{FEED_RATE}\n{CUTTING_PATTERN}\nG00 Z100\nM09 M05".to_string(),
            parameters: vec!["SPINDLE_SPEED".to_string(), "TOOL_NUMBER".to_string(), "START_DEPTH".to_string(), "FEED_RATE".to_string(), "CUTTING_PATTERN".to_string()],
            usage_count: 45,
            last_used: "2024-06-12".to_string(),
        },
        CodeTemplate {
            id: 2,
            name: "ステンレス旋削テンプレート".to_string(),
            description: "SUS304/316L用の精密旋削テンプレート".to_string(),
            machine_type: "CNC旋盤".to_string(),
            operation_type: "旋削加工".to_string(),
            template_code: "G50 S{MAX_SPEED}\nT{TOOL_NUMBER} M06\nG96 S{SURFACE_SPEED} M03\nG00 X{START_X} Z{START_Z}\nG01 X{END_X} F{FEED_RATE}\n{TURNING_PATTERN}\nG00 X100 Z100\nM05".to_string(),
            parameters: vec!["MAX_SPEED".to_string(), "TOOL_NUMBER".to_string(), "SURFACE_SPEED".to_string(), "START_X".to_string(), "START_Z".to_string(), "END_X".to_string(), "FEED_RATE".to_string(), "TURNING_PATTERN".to_string()],
            usage_count: 32,
            last_used: "2024-06-11".to_string(),
        },
        CodeTemplate {
            id: 3,
            name: "穴あけサイクルテンプレート".to_string(),
            description: "高精度穴あけ用の最適化サイクル".to_string(),
            machine_type: "マシニングセンタ".to_string(),
            operation_type: "穴あけ加工".to_string(),
            template_code: "G90 G54\nT{DRILL_NUMBER} M06\nS{SPINDLE_SPEED} M03\nG00 X{HOLE_X} Y{HOLE_Y}\nG43 H{TOOL_NUMBER} Z{CLEARANCE}\nG81 Z{DEPTH} R{RETRACT} F{FEED_RATE}\n{HOLE_PATTERN}\nG80 G00 Z100\nM05".to_string(),
            parameters: vec!["DRILL_NUMBER".to_string(), "SPINDLE_SPEED".to_string(), "HOLE_X".to_string(), "HOLE_Y".to_string(), "TOOL_NUMBER".to_string(), "CLEARANCE".to_string(), "DEPTH".to_string(), "RETRACT".to_string(), "FEED_RATE".to_string(), "HOLE_PATTERN".to_string()],
            usage_count: 67,
            last_used: "2024-06-12".to_string(),
        },
    ]);

    let (active_tab, set_active_tab) = signal("optimizations");
    let (show_optimization_detail, set_show_optimization_detail) = signal(false);
    let (viewing_optimization, set_viewing_optimization) = signal(None::<NcOptimization>);
    let (show_template_detail, set_show_template_detail) = signal(false);
    let (viewing_template, set_viewing_template) = signal(None::<CodeTemplate>);
    let (show_code_generator, set_show_code_generator) = signal(false);

    let view_optimization = move |optimization: NcOptimization| {
        set_viewing_optimization.set(Some(optimization));
        set_show_optimization_detail.set(true);
    };

    let view_template = move |template: CodeTemplate| {
        set_viewing_template.set(Some(template));
        set_show_template_detail.set(true);
    };

    let optimization_type_text = |opt_type: &OptimizationType| match opt_type {
        OptimizationType::SpeedOptimization => "速度最適化",
        OptimizationType::ToolLifeExtension => "工具寿命延長",
        OptimizationType::SurfaceQualityImprovement => "表面品質向上",
        OptimizationType::CuttingPathOptimization => "切削経路最適化",
        OptimizationType::CoolantOptimization => "切削液最適化",
        OptimizationType::PowerConsumptionReduction => "消費電力削減",
    };

    let optimization_type_color = |opt_type: &OptimizationType| match opt_type {
        OptimizationType::SpeedOptimization => "bg-blue-100 text-blue-800",
        OptimizationType::ToolLifeExtension => "bg-green-100 text-green-800",
        OptimizationType::SurfaceQualityImprovement => "bg-purple-100 text-purple-800",
        OptimizationType::CuttingPathOptimization => "bg-orange-100 text-orange-800",
        OptimizationType::CoolantOptimization => "bg-cyan-100 text-cyan-800",
        OptimizationType::PowerConsumptionReduction => "bg-yellow-100 text-yellow-800",
    };

    let status_text = |status: &OptimizationStatus| match status {
        OptimizationStatus::New => "新規",
        OptimizationStatus::Analyzing => "解析中",
        OptimizationStatus::Ready => "適用可能",
        OptimizationStatus::Applied => "適用済み",
        OptimizationStatus::Testing => "テスト中",
        OptimizationStatus::Verified => "検証済み",
    };

    let status_color = |status: &OptimizationStatus| match status {
        OptimizationStatus::New => "bg-gray-100 text-gray-800",
        OptimizationStatus::Analyzing => "bg-yellow-100 text-yellow-800",
        OptimizationStatus::Ready => "bg-green-100 text-green-800",
        OptimizationStatus::Applied => "bg-blue-100 text-blue-800",
        OptimizationStatus::Testing => "bg-purple-100 text-purple-800",
        OptimizationStatus::Verified => "bg-green-100 text-green-800",
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
                            "NCプログラム支援"
                        </h1>
                        <p class="text-slate-300 text-sm">
                            "AIによるNCプログラムの最適化提案と自動コード生成"
                        </p>
                    </div>
                    <button 
                        class="w-full sm:w-auto bg-gradient-to-r from-green-500 to-blue-600 hover:from-green-600 hover:to-blue-700 px-4 sm:px-6 py-3 rounded-lg font-semibold transition-all duration-300 transform hover:scale-105 text-sm sm:text-base"
                        on:click=move |_| set_show_code_generator.set(true)
                    >
                        <span class="sm:hidden">"コード生成"</span>
                        <span class="hidden sm:inline">"AIコード生成"</span>
                    </button>
                </div>

                // Tab Navigation
                <div class="mb-6">
                    <div class="flex flex-wrap gap-2 bg-slate-800/50 backdrop-blur-sm rounded-lg p-2">
                        <button 
                            class={move || format!("px-4 py-2 rounded-md font-medium transition-colors duration-200 {}",
                                if active_tab.get() == "optimizations" { "bg-blue-500 text-white" } else { "text-slate-300 hover:text-white hover:bg-slate-700" }
                            )}
                            on:click=move |_| set_active_tab.set("optimizations")
                        >
                            "最適化提案"
                        </button>
                        <button 
                            class={move || format!("px-4 py-2 rounded-md font-medium transition-colors duration-200 {}",
                                if active_tab.get() == "templates" { "bg-blue-500 text-white" } else { "text-slate-300 hover:text-white hover:bg-slate-700" }
                            )}
                            on:click=move |_| set_active_tab.set("templates")
                        >
                            "コードテンプレート"
                        </button>
                    </div>
                </div>

                // Optimizations Tab
                <Show when=move || active_tab.get() == "optimizations">
                    <div class="space-y-6">
                        // Desktop Table View
                        <div class="hidden lg:block bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 shadow-xl overflow-hidden">
                            <div class="overflow-x-auto">
                                <table class="w-full">
                                    <thead class="bg-slate-700/50">
                                        <tr>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"プログラム名"</th>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"最適化タイプ"</th>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"改善率"</th>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"時間短縮"</th>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"ステータス"</th>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"操作"</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-slate-700/50">
                                        <For
                                            each=move || optimizations.get()
                                            key=|opt| opt.id
                                            children=move |optimization| {
                                                let opt_for_view = optimization.clone();
                                                view! {
                                                    <tr class="hover:bg-slate-700/30 transition-colors duration-200">
                                                        <td class="px-4 py-3 font-medium">{optimization.program_name.clone()}</td>
                                                        <td class="px-4 py-3">
                                                            <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", optimization_type_color(&optimization.optimization_type))}>
                                                                {optimization_type_text(&optimization.optimization_type)}
                                                            </span>
                                                        </td>
                                                        <td class="px-4 py-3 text-slate-300 font-semibold">{format!("{}%", optimization.improvement_percentage)}</td>
                                                        <td class="px-4 py-3 text-slate-300">{optimization.execution_time_reduction.clone()}</td>
                                                        <td class="px-4 py-3">
                                                            <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", status_color(&optimization.status))}>
                                                                {status_text(&optimization.status)}
                                                            </span>
                                                        </td>
                                                        <td class="px-4 py-3">
                                                            <div class="flex gap-1">
                                                                <button 
                                                                    class="bg-blue-500 hover:bg-blue-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200"
                                                                    on:click=move |_| view_optimization(opt_for_view.clone())
                                                                >
                                                                    "詳細"
                                                                </button>
                                                                <button class="bg-green-500 hover:bg-green-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200">
                                                                    "適用"
                                                                </button>
                                                            </div>
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
                                each=move || optimizations.get()
                                key=|opt| opt.id
                                children=move |optimization| {
                                    let opt_for_view = optimization.clone();
                                    view! {
                                        <div class="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 p-4">
                                            <div class="flex justify-between items-start mb-3">
                                                <div class="flex-1">
                                                    <h3 class="font-semibold text-white mb-1">{optimization.program_name.clone()}</h3>
                                                    <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", optimization_type_color(&optimization.optimization_type))}>
                                                        {optimization_type_text(&optimization.optimization_type)}
                                                    </span>
                                                </div>
                                                <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", status_color(&optimization.status))}>
                                                    {status_text(&optimization.status)}
                                                </span>
                                            </div>
                                            
                                            <div class="grid grid-cols-2 gap-2 mb-4 text-sm">
                                                <div>
                                                    <span class="text-slate-400">"改善率: "</span>
                                                    <span class="text-white font-semibold">{format!("{}%", optimization.improvement_percentage)}</span>
                                                </div>
                                                <div>
                                                    <span class="text-slate-400">"時間短縮: "</span>
                                                    <span class="text-slate-300">{optimization.execution_time_reduction.clone()}</span>
                                                </div>
                                            </div>
                                            
                                            <div class="flex gap-2">
                                                <button 
                                                    class="flex-1 bg-blue-500 hover:bg-blue-600 py-2 rounded text-sm font-medium transition-colors duration-200"
                                                    on:click=move |_| view_optimization(opt_for_view.clone())
                                                >
                                                    "詳細を見る"
                                                </button>
                                                <button class="bg-green-500 hover:bg-green-600 px-3 py-2 rounded text-sm font-medium transition-colors duration-200">
                                                    "適用"
                                                </button>
                                            </div>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>
                </Show>

                // Templates Tab
                <Show when=move || active_tab.get() == "templates">
                    <div class="space-y-6">
                        // Desktop Table View
                        <div class="hidden lg:block bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 shadow-xl overflow-hidden">
                            <div class="overflow-x-auto">
                                <table class="w-full">
                                    <thead class="bg-slate-700/50">
                                        <tr>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"テンプレート名"</th>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"機械種別"</th>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"加工タイプ"</th>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"使用回数"</th>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"最終使用"</th>
                                            <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"操作"</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-slate-700/50">
                                        <For
                                            each=move || templates.get()
                                            key=|template| template.id
                                            children=move |template| {
                                                let template_for_view = template.clone();
                                                view! {
                                                    <tr class="hover:bg-slate-700/30 transition-colors duration-200">
                                                        <td class="px-4 py-3 font-medium">{template.name.clone()}</td>
                                                        <td class="px-4 py-3 text-slate-300">{template.machine_type.clone()}</td>
                                                        <td class="px-4 py-3 text-slate-300">{template.operation_type.clone()}</td>
                                                        <td class="px-4 py-3 text-slate-300">{template.usage_count}</td>
                                                        <td class="px-4 py-3 text-slate-300">{template.last_used.clone()}</td>
                                                        <td class="px-4 py-3">
                                                            <div class="flex gap-1">
                                                                <button 
                                                                    class="bg-blue-500 hover:bg-blue-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200"
                                                                    on:click=move |_| view_template(template_for_view.clone())
                                                                >
                                                                    "詳細"
                                                                </button>
                                                                <button class="bg-green-500 hover:bg-green-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200">
                                                                    "使用"
                                                                </button>
                                                            </div>
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
                                each=move || templates.get()
                                key=|template| template.id
                                children=move |template| {
                                    let template_for_view = template.clone();
                                    view! {
                                        <div class="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 p-4">
                                            <div class="flex justify-between items-start mb-3">
                                                <div class="flex-1">
                                                    <h3 class="font-semibold text-white mb-1">{template.name.clone()}</h3>
                                                    <p class="text-slate-300 text-sm">{template.machine_type.clone()}</p>
                                                </div>
                                                <div class="text-right">
                                                    <div class="text-slate-400 text-xs">"使用回数"</div>
                                                    <div class="text-white font-semibold">{template.usage_count}</div>
                                                </div>
                                            </div>
                                            
                                            <div class="grid grid-cols-2 gap-2 mb-4 text-sm">
                                                <div>
                                                    <span class="text-slate-400">"加工タイプ: "</span>
                                                    <span class="text-slate-300">{template.operation_type.clone()}</span>
                                                </div>
                                                <div>
                                                    <span class="text-slate-400">"最終使用: "</span>
                                                    <span class="text-slate-300">{template.last_used.clone()}</span>
                                                </div>
                                            </div>
                                            
                                            <p class="text-slate-400 text-sm mb-4">{template.description.clone()}</p>
                                            
                                            <div class="flex gap-2">
                                                <button 
                                                    class="flex-1 bg-blue-500 hover:bg-blue-600 py-2 rounded text-sm font-medium transition-colors duration-200"
                                                    on:click=move |_| view_template(template_for_view.clone())
                                                >
                                                    "詳細を見る"
                                                </button>
                                                <button class="bg-green-500 hover:bg-green-600 px-3 py-2 rounded text-sm font-medium transition-colors duration-200">
                                                    "使用"
                                                </button>
                                            </div>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>
                </Show>
            </div>

            // Optimization Detail Modal
            <Show when=move || show_optimization_detail.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-6xl mx-4 border border-slate-700 max-h-[90vh] overflow-y-auto">
                        {
                            if let Some(optimization) = viewing_optimization.get() {
                                view! {
                                    <div>
                                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                                            "最適化提案詳細: " {optimization.program_name.clone()}
                                        </h2>
                                        
                                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4">"改善効果"</h3>
                                                <div class="space-y-3">
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-400">"改善率:"</span>
                                                        <span class="text-green-400 font-semibold">{format!("{}%", optimization.improvement_percentage)}</span>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-400">"実行時間短縮:"</span>
                                                        <span class="text-white">{optimization.execution_time_reduction.clone()}</span>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-400">"工具寿命改善:"</span>
                                                        <span class="text-white">{optimization.tool_life_improvement.clone()}</span>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-400">"表面品質改善:"</span>
                                                        <span class="text-white">{optimization.surface_quality_improvement.clone()}</span>
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4">"基本情報"</h3>
                                                <div class="space-y-3">
                                                    <div>
                                                        <span class="text-slate-400">"最適化タイプ: "</span>
                                                        <span class={format!("px-3 py-1 rounded-full text-xs font-medium {}", optimization_type_color(&optimization.optimization_type))}>
                                                            {optimization_type_text(&optimization.optimization_type)}
                                                        </span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"ステータス: "</span>
                                                        <span class={format!("px-3 py-1 rounded-full text-xs font-medium {}", status_color(&optimization.status))}>
                                                            {status_text(&optimization.status)}
                                                        </span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"提案者: "</span>
                                                        <span class="text-white">{optimization.suggested_by.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"作成日: "</span>
                                                        <span class="text-white">{optimization.created_date.clone()}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4">"元のコード"</h3>
                                                <pre class="bg-slate-700/50 p-4 rounded-lg text-slate-300 text-sm overflow-x-auto">
                                                    <code>{optimization.original_code.clone()}</code>
                                                </pre>
                                            </div>
                                            
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4">"最適化されたコード"</h3>
                                                <pre class="bg-green-900/20 border border-green-500/30 p-4 rounded-lg text-green-300 text-sm overflow-x-auto">
                                                    <code>{optimization.optimized_code.clone()}</code>
                                                </pre>
                                            </div>
                                        </div>
                                        
                                        <div class="flex gap-3">
                                            <button class="bg-green-500 hover:bg-green-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "最適化を適用"
                                            </button>
                                            <button class="bg-blue-500 hover:bg-blue-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "テスト実行"
                                            </button>
                                            <button class="bg-orange-500 hover:bg-orange-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "コードをダウンロード"
                                            </button>
                                            <button 
                                                class="bg-slate-600 hover:bg-slate-500 px-4 py-2 rounded-lg font-medium transition-colors duration-200"
                                                on:click=move |_| {
                                                    set_viewing_optimization.set(None);
                                                    set_show_optimization_detail.set(false);
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

            // Template Detail Modal
            <Show when=move || show_template_detail.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-4xl mx-4 border border-slate-700 max-h-[90vh] overflow-y-auto">
                        {
                            if let Some(template) = viewing_template.get() {
                                view! {
                                    <div>
                                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                                            "テンプレート詳細: " {template.name.clone()}
                                        </h2>
                                        
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4">"基本情報"</h3>
                                                <div class="space-y-3">
                                                    <div>
                                                        <span class="text-slate-400">"機械種別: "</span>
                                                        <span class="text-white">{template.machine_type.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"加工タイプ: "</span>
                                                        <span class="text-white">{template.operation_type.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"使用回数: "</span>
                                                        <span class="text-white font-semibold">{template.usage_count}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"最終使用: "</span>
                                                        <span class="text-white">{template.last_used.clone()}</span>
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4">"パラメータ"</h3>
                                                <div class="flex flex-wrap gap-2">
                                                    <For
                                                        each=move || template.parameters.clone()
                                                        key=|param| param.clone()
                                                        children=move |param| {
                                                            view! {
                                                                <span class="px-2 py-1 bg-blue-600/20 border border-blue-500/30 text-blue-300 rounded text-sm">
                                                                    {param}
                                                                </span>
                                                            }
                                                        }
                                                    />
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="mb-6">
                                            <h3 class="text-lg font-semibold mb-4">"説明"</h3>
                                            <p class="text-slate-300 bg-slate-700/50 p-4 rounded-lg">
                                                {template.description.clone()}
                                            </p>
                                        </div>
                                        
                                        <div class="mb-6">
                                            <h3 class="text-lg font-semibold mb-4">"テンプレートコード"</h3>
                                            <pre class="bg-slate-700/50 p-4 rounded-lg text-slate-300 text-sm overflow-x-auto">
                                                <code>{template.template_code.clone()}</code>
                                            </pre>
                                        </div>
                                        
                                        <div class="flex gap-3">
                                            <button class="bg-green-500 hover:bg-green-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "テンプレートを使用"
                                            </button>
                                            <button class="bg-blue-500 hover:bg-blue-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "コピー"
                                            </button>
                                            <button class="bg-orange-500 hover:bg-orange-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "ダウンロード"
                                            </button>
                                            <button 
                                                class="bg-slate-600 hover:bg-slate-500 px-4 py-2 rounded-lg font-medium transition-colors duration-200"
                                                on:click=move |_| {
                                                    set_viewing_template.set(None);
                                                    set_show_template_detail.set(false);
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

            // AI Code Generator Modal
            <Show when=move || show_code_generator.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-2xl mx-4 border border-slate-700 max-h-[90vh] overflow-y-auto">
                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-green-400 to-blue-400 bg-clip-text text-transparent">
                            "AIコード生成"
                        </h2>
                        
                        <div class="space-y-4 mb-6">
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"加工内容"</label>
                                <textarea 
                                    rows="3"
                                    placeholder="例: アルミ材の10mm穴を20箇所開ける加工"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                ></textarea>
                            </div>
                            
                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="block text-sm font-medium text-slate-300 mb-2">"機械種別"</label>
                                    <select class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500">
                                        <option value="">"選択してください"</option>
                                        <option value="machining_center">"マシニングセンタ"</option>
                                        <option value="lathe">"旋盤"</option>
                                        <option value="drilling">"ボール盤"</option>
                                    </select>
                                </div>
                                
                                <div>
                                    <label class="block text-sm font-medium text-slate-300 mb-2">"材質"</label>
                                    <input 
                                        type="text"
                                        placeholder="A5052, SUS304など"
                                        class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    />
                                </div>
                            </div>
                        </div>
                        
                        <div class="flex gap-3">
                            <button class="flex-1 bg-gradient-to-r from-green-500 to-blue-600 hover:from-green-600 hover:to-blue-700 py-2 rounded-lg font-medium transition-all duration-300">
                                "コード生成"
                            </button>
                            <button 
                                class="bg-slate-600 hover:bg-slate-500 px-4 py-2 rounded-lg font-medium transition-colors duration-200"
                                on:click=move |_| set_show_code_generator.set(false)
                            >
                                "キャンセル"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}