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
                    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                        "工具管理"
                    </h1>
                    <button 
                        class="w-full sm:w-auto bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 px-4 sm:px-6 py-3 rounded-lg font-semibold transition-all duration-300 transform hover:scale-105 text-sm sm:text-base"
                        on:click=move |_| set_show_add_modal.set(true)
                    >
                        <span class="sm:hidden">"新規追加"</span>
                        <span class="hidden sm:inline">"新規工具追加"</span>
                    </button>
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
                                    each=move || tools.get()
                                    key=|tool| tool.id
                                    children=move |tool| {
                                        let tool_id = tool.id;
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
                                                    <button 
                                                        class="bg-red-500 hover:bg-red-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200"
                                                        on:click=move |_| delete_tool(tool_id)
                                                    >
                                                        "削除"
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
                        each=move || tools.get()
                        key=|tool| tool.id
                        children=move |tool| {
                            let tool_id = tool.id;
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
        </div>
    }
}