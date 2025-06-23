use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct Tool {
    pub id: u32,
    pub name: String,
    pub tool_type: String,
    pub status: ToolStatus,
    pub location: String,
    pub last_maintenance: String,
    pub next_maintenance: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ToolStatus {
    Available,
    InUse,
    Maintenance,
    Damaged,
}

#[component]
pub fn ToolManagement() -> impl IntoView {
    let (tools, set_tools) = signal(vec![
        Tool {
            id: 1,
            name: "エンドミル φ10mm".to_string(),
            tool_type: "切削工具".to_string(),
            status: ToolStatus::Available,
            location: "工具棚A-1".to_string(),
            last_maintenance: "2024-05-15".to_string(),
            next_maintenance: "2024-08-15".to_string(),
        },
        Tool {
            id: 2,
            name: "ドリル φ6.5mm".to_string(),
            tool_type: "穴あけ工具".to_string(),
            status: ToolStatus::InUse,
            location: "MC-001".to_string(),
            last_maintenance: "2024-04-20".to_string(),
            next_maintenance: "2024-07-20".to_string(),
        },
        Tool {
            id: 3,
            name: "フェイスミル φ50mm".to_string(),
            tool_type: "面削り工具".to_string(),
            status: ToolStatus::Maintenance,
            location: "メンテナンス室".to_string(),
            last_maintenance: "2024-06-01".to_string(),
            next_maintenance: "2024-09-01".to_string(),
        },
        Tool {
            id: 4,
            name: "タップ M8".to_string(),
            tool_type: "ねじ切り工具".to_string(),
            status: ToolStatus::Damaged,
            location: "修理待ち".to_string(),
            last_maintenance: "2024-03-10".to_string(),
            next_maintenance: "要修理".to_string(),
        },
    ]);

    let (show_add_modal, set_show_add_modal) = signal(false);
    let (show_detail_modal, set_show_detail_modal) = signal(false);
    let (viewing_tool, set_viewing_tool) = signal(None::<Tool>);
    let (search_query, set_search_query) = signal(String::new());
    let (status_filter, set_status_filter) = signal(None::<ToolStatus>);
    let (new_tool_name, set_new_tool_name) = signal(String::new());
    let (new_tool_type, set_new_tool_type) = signal(String::new());
    let (new_tool_location, set_new_tool_location) = signal(String::new());

    let add_tool = move |_| {
        if !new_tool_name.get().is_empty() {
            let new_tool = Tool {
                id: tools.get().len() as u32 + 1,
                name: new_tool_name.get(),
                tool_type: new_tool_type.get(),
                status: ToolStatus::Available,
                location: new_tool_location.get(),
                last_maintenance: "2024-06-12".to_string(),
                next_maintenance: "2024-09-12".to_string(),
            };
            set_tools.update(|tools| tools.push(new_tool));
            set_new_tool_name.set(String::new());
            set_new_tool_type.set(String::new());
            set_new_tool_location.set(String::new());
            set_show_add_modal.set(false);
        }
    };

    let delete_tool = move |id: u32| {
        set_tools.update(|tools| {
            tools.retain(|tool| tool.id != id);
        });
    };

    let view_tool = move |tool: Tool| {
        set_viewing_tool.set(Some(tool));
        set_show_detail_modal.set(true);
    };

    let filtered_tools = move || {
        tools.get().into_iter().filter(|tool| {
            let search_match = search_query.get().is_empty() || 
                tool.name.to_lowercase().contains(&search_query.get().to_lowercase()) ||
                tool.tool_type.to_lowercase().contains(&search_query.get().to_lowercase()) ||
                tool.location.to_lowercase().contains(&search_query.get().to_lowercase());
            
            let status_match = status_filter.get().is_none() || status_filter.get() == Some(tool.status.clone());
            
            search_match && status_match
        }).collect::<Vec<_>>()
    };

    let tool_stats = move || {
        let all_tools = tools.get();
        let total = all_tools.len();
        let available = all_tools.iter().filter(|t| t.status == ToolStatus::Available).count();
        let in_use = all_tools.iter().filter(|t| t.status == ToolStatus::InUse).count();
        let maintenance = all_tools.iter().filter(|t| t.status == ToolStatus::Maintenance).count();
        let damaged = all_tools.iter().filter(|t| t.status == ToolStatus::Damaged).count();
        
        (total, available, in_use, maintenance, damaged)
    };

    let status_color = |status: &ToolStatus| match status {
        ToolStatus::Available => "bg-green-100 text-green-800",
        ToolStatus::InUse => "bg-blue-100 text-blue-800", 
        ToolStatus::Maintenance => "bg-yellow-100 text-yellow-800",
        ToolStatus::Damaged => "bg-red-100 text-red-800",
    };

    let status_text = |status: &ToolStatus| match status {
        ToolStatus::Available => "利用可能",
        ToolStatus::InUse => "使用中",
        ToolStatus::Maintenance => "メンテナンス中",
        ToolStatus::Damaged => "故障",
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
                            "工具管理"
                        </h1>
                        <p class="text-slate-300 text-sm">"工具の在庫、ステータス、メンテナンス情報を管理"</p>
                    </div>
                    <button 
                        class="w-full sm:w-auto bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 px-4 sm:px-6 py-3 rounded-lg font-semibold transition-all duration-300 transform hover:scale-105 text-sm sm:text-base"
                        on:click=move |_| set_show_add_modal.set(true)
                    >
                        <span class="sm:hidden">"新規追加"</span>
                        <span class="hidden sm:inline">"新規工具追加"</span>
                    </button>
                </div>

                // Statistics Cards
                <div class="grid grid-cols-2 md:grid-cols-5 gap-4 mb-8">
                    {
                        let (total, available, in_use, maintenance, damaged) = tool_stats();
                        view! {
                            <>
                                <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-4 border border-slate-700">
                                    <div class="text-2xl font-bold text-white mb-1">{total}</div>
                                    <div class="text-sm text-slate-400">"総工具数"</div>
                                </div>
                                <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-4 border border-slate-700">
                                    <div class="text-2xl font-bold text-green-400 mb-1">{available}</div>
                                    <div class="text-sm text-slate-400">"利用可能"</div>
                                </div>
                                <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-4 border border-slate-700">
                                    <div class="text-2xl font-bold text-blue-400 mb-1">{in_use}</div>
                                    <div class="text-sm text-slate-400">"使用中"</div>
                                </div>
                                <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-4 border border-slate-700">
                                    <div class="text-2xl font-bold text-yellow-400 mb-1">{maintenance}</div>
                                    <div class="text-sm text-slate-400">"メンテナンス"</div>
                                </div>
                                <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-4 border border-slate-700">
                                    <div class="text-2xl font-bold text-red-400 mb-1">{damaged}</div>
                                    <div class="text-sm text-slate-400">"故障"</div>
                                </div>
                            </>
                        }
                    }
                </div>

                // Search and Filter Section
                <div class="bg-slate-800/50 backdrop-blur-lg rounded-xl p-6 border border-slate-700 mb-8">
                    <div class="flex flex-col lg:flex-row gap-4">
                        // Search Input
                        <div class="flex-1">
                            <label class="block text-sm font-medium text-slate-300 mb-2">"検索"</label>
                            <input 
                                type="text"
                                placeholder="工具名、種類、保管場所で検索..."
                                class="w-full px-4 py-3 bg-slate-700/50 border border-slate-600 rounded-lg text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                prop:value=move || search_query.get()
                                on:input=move |ev| set_search_query.set(event_target_value(&ev))
                            />
                        </div>

                        // Status Filter
                        <div class="lg:w-64">
                            <label class="block text-sm font-medium text-slate-300 mb-2">"ステータスフィルタ"</label>
                            <select 
                                class="w-full px-4 py-3 bg-slate-700/50 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    match value.as_str() {
                                        "available" => set_status_filter.set(Some(ToolStatus::Available)),
                                        "in_use" => set_status_filter.set(Some(ToolStatus::InUse)),
                                        "maintenance" => set_status_filter.set(Some(ToolStatus::Maintenance)),
                                        "damaged" => set_status_filter.set(Some(ToolStatus::Damaged)),
                                        _ => set_status_filter.set(None),
                                    }
                                }
                            >
                                <option value="">"すべて"</option>
                                <option value="available">"利用可能"</option>
                                <option value="in_use">"使用中"</option>
                                <option value="maintenance">"メンテナンス中"</option>
                                <option value="damaged">"故障"</option>
                            </select>
                        </div>

                        // Clear Filters Button
                        <div class="lg:w-32 flex items-end">
                            <button 
                                class="w-full bg-slate-600 hover:bg-slate-500 px-4 py-3 rounded-lg font-medium transition-colors duration-200"
                                on:click=move |_| {
                                    set_search_query.set(String::new());
                                    set_status_filter.set(None);
                                }
                            >
                                "クリア"
                            </button>
                        </div>
                    </div>
                </div>

                // Desktop Table View
                <div class="hidden lg:block bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 shadow-xl overflow-hidden">
                    <div class="overflow-x-auto">
                        <table class="w-full">
                            <thead class="bg-slate-700/50">
                                <tr>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"工具名"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"種類"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"ステータス"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"保管場所"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"前回メンテナンス"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"次回メンテナンス"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"操作"</th>
                                </tr>
                            </thead>
                            <tbody class="divide-y divide-slate-700/50">
                                <For
                                    each=filtered_tools
                                    key=|tool| tool.id
                                    children=move |tool| {
                                        let tool_id = tool.id;
                                        let tool_for_view = tool.clone();
                                        view! {
                                            <tr class="hover:bg-slate-700/30 transition-colors duration-200">
                                                <td class="px-4 py-3 font-medium">{tool.name.clone()}</td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{tool.tool_type.clone()}</td>
                                                <td class="px-4 py-3">
                                                    <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", status_color(&tool.status))}>
                                                        {status_text(&tool.status)}
                                                    </span>
                                                </td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{tool.location.clone()}</td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{tool.last_maintenance.clone()}</td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{tool.next_maintenance.clone()}</td>
                                                <td class="px-4 py-3">
                                                    <div class="flex gap-1">
                                                        <button 
                                                            class="bg-blue-500 hover:bg-blue-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200"
                                                            on:click=move |_| view_tool(tool_for_view.clone())
                                                        >
                                                            "詳細"
                                                        </button>
                                                        <button 
                                                            class="bg-red-500 hover:bg-red-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200"
                                                            on:click=move |_| delete_tool(tool_id)
                                                        >
                                                            "削除"
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
                        each=filtered_tools
                        key=|tool| tool.id
                        children=move |tool| {
                            let tool_id = tool.id;
                            let tool_for_view = tool.clone();
                            view! {
                                <div class="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 p-4">
                                    <div class="flex justify-between items-start mb-3">
                                        <div>
                                            <h3 class="font-semibold text-white mb-1">{tool.name.clone()}</h3>
                                            <p class="text-slate-300 text-sm">{tool.tool_type.clone()}</p>
                                        </div>
                                        <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", status_color(&tool.status))}>
                                            {status_text(&tool.status)}
                                        </span>
                                    </div>
                                    
                                    <div class="grid grid-cols-2 gap-2 mb-4 text-sm">
                                        <div>
                                            <span class="text-slate-400">"保管場所: "</span>
                                            <span class="text-slate-300">{tool.location.clone()}</span>
                                        </div>
                                        <div>
                                            <span class="text-slate-400">"前回メンテ: "</span>
                                            <span class="text-slate-300">{tool.last_maintenance.clone()}</span>
                                        </div>
                                    </div>
                                    
                                    <div class="mb-3">
                                        <span class="text-slate-400 text-sm">"次回メンテ: "</span>
                                        <span class="text-slate-300 text-sm">{tool.next_maintenance.clone()}</span>
                                    </div>
                                    
                                    <div class="flex gap-2">
                                        <button 
                                            class="flex-1 bg-blue-500 hover:bg-blue-600 py-2 rounded text-sm font-medium transition-colors duration-200"
                                            on:click=move |_| view_tool(tool_for_view.clone())
                                        >
                                            "詳細"
                                        </button>
                                        <button 
                                            class="bg-red-500 hover:bg-red-600 px-3 py-2 rounded text-sm font-medium transition-colors duration-200"
                                            on:click=move |_| delete_tool(tool_id)
                                        >
                                            "削除"
                                        </button>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </div>

            // Add Tool Modal
            <Show when=move || show_add_modal.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-md mx-4 border border-slate-700">
                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                            "新規工具追加"
                        </h2>
                        
                        <div class="space-y-4">
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"工具名"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_tool_name.get()
                                    on:input=move |ev| set_new_tool_name.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"種類"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_tool_type.get()
                                    on:input=move |ev| set_new_tool_type.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"保管場所"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_tool_location.get()
                                    on:input=move |ev| set_new_tool_location.set(event_target_value(&ev))
                                />
                            </div>
                        </div>
                        
                        <div class="flex gap-3 mt-6">
                            <button 
                                class="flex-1 bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 py-2 rounded-lg font-medium transition-all duration-300"
                                on:click=add_tool
                            >
                                "追加"
                            </button>
                            <button 
                                class="flex-1 bg-slate-600 hover:bg-slate-500 py-2 rounded-lg font-medium transition-colors duration-200"
                                on:click=move |_| {
                                    set_new_tool_name.set(String::new());
                                    set_new_tool_type.set(String::new());
                                    set_new_tool_location.set(String::new());
                                    set_show_add_modal.set(false);
                                }
                            >
                                "キャンセル"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>

            // Tool Detail Modal
            <Show when=move || show_detail_modal.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-2xl mx-4 border border-slate-700 max-h-[90vh] overflow-y-auto">
                        {
                            if let Some(tool) = viewing_tool.get() {
                                view! {
                                    <div>
                                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                                            "工具詳細: " {tool.name.clone()}
                                        </h2>
                                        
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4 text-white">"基本情報"</h3>
                                                <div class="space-y-3">
                                                    <div>
                                                        <span class="text-slate-400">"工具名: "</span>
                                                        <span class="text-white font-medium">{tool.name.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"種類: "</span>
                                                        <span class="text-white">{tool.tool_type.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"ステータス: "</span>
                                                        <span class={format!("px-3 py-1 rounded-full text-xs font-medium {}", status_color(&tool.status))}>
                                                            {status_text(&tool.status)}
                                                        </span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"保管場所: "</span>
                                                        <span class="text-white">{tool.location.clone()}</span>
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4 text-white">"メンテナンス情報"</h3>
                                                <div class="space-y-3">
                                                    <div>
                                                        <span class="text-slate-400">"前回メンテナンス: "</span>
                                                        <span class="text-white">{tool.last_maintenance.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"次回メンテナンス: "</span>
                                                        <span class="text-white">{tool.next_maintenance.clone()}</span>
                                                    </div>
                                                    <div class="mt-4">
                                                        <div class="text-sm text-slate-400 mb-2">"メンテナンス状況"</div>
                                                        <div class="w-full bg-slate-700 rounded-full h-2">
                                                            <div 
                                                                class="bg-gradient-to-r from-green-500 to-yellow-500 h-2 rounded-full"
                                                                style="width: 65%"
                                                            ></div>
                                                        </div>
                                                        <div class="text-xs text-slate-500 mt-1">"あと35日でメンテナンス期限"</div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="mb-6">
                                            <h3 class="text-lg font-semibold mb-4 text-white">"使用履歴"</h3>
                                            <div class="bg-slate-700/50 p-4 rounded-lg">
                                                <div class="space-y-2 text-sm">
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-400">"総使用時間:"</span>
                                                        <span class="text-white">"245時間"</span>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-400">"今月の使用時間:"</span>
                                                        <span class="text-white">"32時間"</span>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-400">"最終使用日:"</span>
                                                        <span class="text-white">"2024-06-11"</span>
                                                    </div>
                                                    <div class="flex justify-between">
                                                        <span class="text-slate-400">"推定寿命:"</span>
                                                        <span class="text-white">"あと580時間"</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="flex gap-3">
                                            <button class="bg-green-500 hover:bg-green-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "メンテナンス実行"
                                            </button>
                                            <button class="bg-blue-500 hover:bg-blue-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "ステータス変更"
                                            </button>
                                            <button class="bg-orange-500 hover:bg-orange-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "履歴を表示"
                                            </button>
                                            <button 
                                                class="bg-slate-600 hover:bg-slate-500 px-4 py-2 rounded-lg font-medium transition-colors duration-200"
                                                on:click=move |_| {
                                                    set_viewing_tool.set(None);
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