use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct NcProgram {
    pub id: u32,
    pub name: String,
    pub program_number: String,
    pub version: String,
    pub machine_type: String,
    pub material: String,
    pub operation: String,
    pub created_by: String,
    pub created_date: String,
    pub last_modified: String,
    pub status: ProgramStatus,
    pub file_size: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProgramStatus {
    Active,
    Testing,
    Archived,
    Deprecated,
}

#[component]
pub fn NcProgramManagement() -> impl IntoView {
    let (programs, set_programs) = signal(vec![
        NcProgram {
            id: 1,
            name: "アルミブラケット加工".to_string(),
            program_number: "O1001".to_string(),
            version: "v2.1".to_string(),
            machine_type: "マシニングセンタ".to_string(),
            material: "A5052".to_string(),
            operation: "フライス加工".to_string(),
            created_by: "田中太郎".to_string(),
            created_date: "2024-01-15".to_string(),
            last_modified: "2024-05-20".to_string(),
            status: ProgramStatus::Active,
            file_size: "15.2KB".to_string(),
            description: "アルミブラケット用の標準加工プログラム".to_string(),
        },
        NcProgram {
            id: 2,
            name: "ステンレスパイプ切削".to_string(),
            program_number: "O2003".to_string(),
            version: "v1.5".to_string(),
            machine_type: "旋盤".to_string(),
            material: "SUS304".to_string(),
            operation: "旋削加工".to_string(),
            created_by: "佐藤花子".to_string(),
            created_date: "2024-02-10".to_string(),
            last_modified: "2024-06-01".to_string(),
            status: ProgramStatus::Testing,
            file_size: "8.7KB".to_string(),
            description: "ステンレスパイプの精密切削プログラム".to_string(),
        },
        NcProgram {
            id: 3,
            name: "鉄鋼プレート穴あけ".to_string(),
            program_number: "O3005".to_string(),
            version: "v3.0".to_string(),
            machine_type: "ボール盤".to_string(),
            material: "SS400".to_string(),
            operation: "穴あけ加工".to_string(),
            created_by: "山田次郎".to_string(),
            created_date: "2023-11-08".to_string(),
            last_modified: "2024-03-15".to_string(),
            status: ProgramStatus::Archived,
            file_size: "12.4KB".to_string(),
            description: "鉄鋼プレート用の多穴加工プログラム".to_string(),
        },
        NcProgram {
            id: 4,
            name: "旧版ギア加工".to_string(),
            program_number: "O9999".to_string(),
            version: "v1.0".to_string(),
            machine_type: "マシニングセンタ".to_string(),
            material: "SS400".to_string(),
            operation: "ギア加工".to_string(),
            created_by: "旧システム".to_string(),
            created_date: "2020-01-01".to_string(),
            last_modified: "2020-01-01".to_string(),
            status: ProgramStatus::Deprecated,
            file_size: "5.2KB".to_string(),
            description: "廃止予定の旧版ギア加工プログラム".to_string(),
        },
    ]);

    let (show_add_modal, set_show_add_modal) = signal(false);
    let (show_view_modal, set_show_view_modal) = signal(false);
    let (viewing_program, set_viewing_program) = signal(None::<NcProgram>);
    
    let (new_name, set_new_name) = signal(String::new());
    let (new_program_number, set_new_program_number) = signal(String::new());
    let (new_machine_type, set_new_machine_type) = signal(String::new());
    let (new_material, set_new_material) = signal(String::new());
    let (new_operation, set_new_operation) = signal(String::new());
    let (new_description, set_new_description) = signal(String::new());

    let clear_form = move || {
        set_new_name.set(String::new());
        set_new_program_number.set(String::new());
        set_new_machine_type.set(String::new());
        set_new_material.set(String::new());
        set_new_operation.set(String::new());
        set_new_description.set(String::new());
    };

    let add_program = move |_| {
        if !new_name.get().is_empty() && !new_program_number.get().is_empty() {
            let new_program = NcProgram {
                id: programs.get().len() as u32 + 1,
                name: new_name.get(),
                program_number: new_program_number.get(),
                version: "v1.0".to_string(),
                machine_type: new_machine_type.get(),
                material: new_material.get(),
                operation: new_operation.get(),
                created_by: "現在のユーザー".to_string(),
                created_date: "2024-06-12".to_string(),
                last_modified: "2024-06-12".to_string(),
                status: ProgramStatus::Testing,
                file_size: "10.0KB".to_string(),
                description: new_description.get(),
            };
            set_programs.update(|programs| programs.push(new_program));
            clear_form();
            set_show_add_modal.set(false);
        }
    };

    let view_program = move |program: NcProgram| {
        set_viewing_program.set(Some(program));
        set_show_view_modal.set(true);
    };

    let delete_program = move |id: u32| {
        set_programs.update(|programs| {
            programs.retain(|program| program.id != id);
        });
    };

    let status_color = |status: &ProgramStatus| match status {
        ProgramStatus::Active => "bg-green-100 text-green-800",
        ProgramStatus::Testing => "bg-yellow-100 text-yellow-800",
        ProgramStatus::Archived => "bg-blue-100 text-blue-800",
        ProgramStatus::Deprecated => "bg-red-100 text-red-800",
    };

    let status_text = |status: &ProgramStatus| match status {
        ProgramStatus::Active => "運用中",
        ProgramStatus::Testing => "テスト中",
        ProgramStatus::Archived => "アーカイブ",
        ProgramStatus::Deprecated => "廃止",
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
                        "NCプログラム管理"
                    </h1>
                    <button 
                        class="w-full sm:w-auto bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 px-4 sm:px-6 py-3 rounded-lg font-semibold transition-all duration-300 transform hover:scale-105 text-sm sm:text-base"
                        on:click=move |_| set_show_add_modal.set(true)
                    >
                        <span class="sm:hidden">"新規追加"</span>
                        <span class="hidden sm:inline">"新規プログラム追加"</span>
                    </button>
                </div>

                // Desktop Table View
                <div class="hidden lg:block bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 shadow-xl overflow-hidden">
                    <div class="overflow-x-auto">
                        <table class="w-full">
                            <thead class="bg-slate-700/50">
                                <tr>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"プログラム名"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"番号"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"バージョン"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"機械種別"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"材質"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"作成者"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"最終更新"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"ステータス"</th>
                                    <th class="px-4 py-3 text-left text-sm font-semibold text-slate-300">"操作"</th>
                                </tr>
                            </thead>
                            <tbody class="divide-y divide-slate-700/50">
                                <For
                                    each=move || programs.get()
                                    key=|program| program.id
                                    children=move |program| {
                                        let program_id = program.id;
                                        let program_for_view = program.clone();
                                        view! {
                                            <tr class="hover:bg-slate-700/30 transition-colors duration-200">
                                                <td class="px-4 py-3 font-medium">{program.name.clone()}</td>
                                                <td class="px-4 py-3 text-slate-300 font-mono text-sm">{program.program_number.clone()}</td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{program.version.clone()}</td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{program.machine_type.clone()}</td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{program.material.clone()}</td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{program.created_by.clone()}</td>
                                                <td class="px-4 py-3 text-slate-300 text-sm">{program.last_modified.clone()}</td>
                                                <td class="px-4 py-3">
                                                    <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", status_color(&program.status))}>
                                                        {status_text(&program.status)}
                                                    </span>
                                                </td>
                                                <td class="px-4 py-3">
                                                    <div class="flex gap-1">
                                                        <button 
                                                            class="bg-blue-500 hover:bg-blue-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200"
                                                            on:click=move |_| view_program(program_for_view.clone())
                                                        >
                                                            "詳細"
                                                        </button>
                                                        <button 
                                                            class="bg-green-500 hover:bg-green-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200"
                                                        >
                                                            "DL"
                                                        </button>
                                                        <button 
                                                            class="bg-red-500 hover:bg-red-600 px-2 py-1 rounded text-xs font-medium transition-colors duration-200"
                                                            on:click=move |_| delete_program(program_id)
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
                        each=move || programs.get()
                        key=|program| program.id
                        children=move |program| {
                            let program_id = program.id;
                            let program_for_view = program.clone();
                            view! {
                                <div class="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 p-4">
                                    <div class="flex justify-between items-start mb-3">
                                        <div>
                                            <h3 class="font-semibold text-white mb-1">{program.name.clone()}</h3>
                                            <p class="text-slate-300 font-mono text-sm">{program.program_number.clone()}</p>
                                        </div>
                                        <span class={format!("px-2 py-1 rounded-full text-xs font-medium {}", status_color(&program.status))}>
                                            {status_text(&program.status)}
                                        </span>
                                    </div>
                                    
                                    <div class="grid grid-cols-2 gap-2 mb-4 text-sm">
                                        <div>
                                            <span class="text-slate-400">"バージョン: "</span>
                                            <span class="text-slate-300">{program.version.clone()}</span>
                                        </div>
                                        <div>
                                            <span class="text-slate-400">"機械: "</span>
                                            <span class="text-slate-300">{program.machine_type.clone()}</span>
                                        </div>
                                        <div>
                                            <span class="text-slate-400">"材質: "</span>
                                            <span class="text-slate-300">{program.material.clone()}</span>
                                        </div>
                                        <div>
                                            <span class="text-slate-400">"作成者: "</span>
                                            <span class="text-slate-300">{program.created_by.clone()}</span>
                                        </div>
                                    </div>
                                    
                                    <div class="mb-3">
                                        <span class="text-slate-400 text-sm">"最終更新: "</span>
                                        <span class="text-slate-300 text-sm">{program.last_modified.clone()}</span>
                                    </div>
                                    
                                    <div class="flex gap-2">
                                        <button 
                                            class="flex-1 bg-blue-500 hover:bg-blue-600 py-2 rounded text-sm font-medium transition-colors duration-200"
                                            on:click=move |_| view_program(program_for_view.clone())
                                        >
                                            "詳細"
                                        </button>
                                        <button class="bg-green-500 hover:bg-green-600 px-3 py-2 rounded text-sm font-medium transition-colors duration-200">
                                            "DL"
                                        </button>
                                        <button 
                                            class="bg-red-500 hover:bg-red-600 px-3 py-2 rounded text-sm font-medium transition-colors duration-200"
                                            on:click=move |_| delete_program(program_id)
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

            // Add Program Modal
            <Show when=move || show_add_modal.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-2xl mx-4 border border-slate-700 max-h-[90vh] overflow-y-auto">
                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                            "新規NCプログラム追加"
                        </h2>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"プログラム名"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_name.get()
                                    on:input=move |ev| set_new_name.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"プログラム番号"</label>
                                <input 
                                    type="text"
                                    placeholder="O1000"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_program_number.get()
                                    on:input=move |ev| set_new_program_number.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"機械種別"</label>
                                <select 
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    on:change=move |ev| set_new_machine_type.set(event_target_value(&ev))
                                >
                                    <option value="">"選択してください"</option>
                                    <option value="マシニングセンタ">"マシニングセンタ"</option>
                                    <option value="旋盤">"旋盤"</option>
                                    <option value="ボール盤">"ボール盤"</option>
                                    <option value="フライス盤">"フライス盤"</option>
                                    <option value="研削盤">"研削盤"</option>
                                </select>
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"材質"</label>
                                <input 
                                    type="text"
                                    placeholder="A5052, SUS304, SS400 など"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_material.get()
                                    on:input=move |ev| set_new_material.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div class="md:col-span-2">
                                <label class="block text-sm font-medium text-slate-300 mb-2">"加工内容"</label>
                                <input 
                                    type="text"
                                    placeholder="フライス加工、旋削加工、穴あけ加工 など"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_operation.get()
                                    on:input=move |ev| set_new_operation.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div class="md:col-span-2">
                                <label class="block text-sm font-medium text-slate-300 mb-2">"説明"</label>
                                <textarea 
                                    rows="3"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_description.get()
                                    on:input=move |ev| set_new_description.set(event_target_value(&ev))
                                ></textarea>
                            </div>
                        </div>
                        
                        <div class="flex gap-3 mt-6">
                            <button 
                                class="flex-1 bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 py-2 rounded-lg font-medium transition-all duration-300"
                                on:click=add_program
                            >
                                "追加"
                            </button>
                            <button 
                                class="flex-1 bg-slate-600 hover:bg-slate-500 py-2 rounded-lg font-medium transition-colors duration-200"
                                on:click=move |_| {
                                    clear_form();
                                    set_show_add_modal.set(false);
                                }
                            >
                                "キャンセル"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>

            // View Program Details Modal
            <Show when=move || show_view_modal.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-3xl mx-4 border border-slate-700 max-h-[90vh] overflow-y-auto">
                        {
                            if let Some(program) = viewing_program.get() {
                                view! {
                                    <div>
                                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                                            "プログラム詳細"
                                        </h2>
                                        
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4">"基本情報"</h3>
                                                <div class="space-y-3">
                                                    <div>
                                                        <span class="text-slate-400">"プログラム名: "</span>
                                                        <span class="text-white">{program.name.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"プログラム番号: "</span>
                                                        <span class="text-white font-mono">{program.program_number.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"バージョン: "</span>
                                                        <span class="text-white">{program.version.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"ファイルサイズ: "</span>
                                                        <span class="text-white">{program.file_size.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"ステータス: "</span>
                                                        <span class={format!("px-3 py-1 rounded-full text-xs font-medium {}", status_color(&program.status))}>
                                                            {status_text(&program.status)}
                                                        </span>
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div>
                                                <h3 class="text-lg font-semibold mb-4">"加工情報"</h3>
                                                <div class="space-y-3">
                                                    <div>
                                                        <span class="text-slate-400">"機械種別: "</span>
                                                        <span class="text-white">{program.machine_type.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"材質: "</span>
                                                        <span class="text-white">{program.material.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"加工内容: "</span>
                                                        <span class="text-white">{program.operation.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"作成者: "</span>
                                                        <span class="text-white">{program.created_by.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"作成日: "</span>
                                                        <span class="text-white">{program.created_date.clone()}</span>
                                                    </div>
                                                    <div>
                                                        <span class="text-slate-400">"最終更新: "</span>
                                                        <span class="text-white">{program.last_modified.clone()}</span>
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div class="md:col-span-2">
                                                <h3 class="text-lg font-semibold mb-4">"説明"</h3>
                                                <p class="text-slate-300 bg-slate-700/50 p-4 rounded-lg">
                                                    {program.description.clone()}
                                                </p>
                                            </div>
                                        </div>
                                        
                                        <div class="flex gap-3 mt-6">
                                            <button class="bg-green-500 hover:bg-green-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "ダウンロード"
                                            </button>
                                            <button class="bg-blue-500 hover:bg-blue-600 px-4 py-2 rounded-lg font-medium transition-colors duration-200">
                                                "編集"
                                            </button>
                                            <button 
                                                class="bg-slate-600 hover:bg-slate-500 px-4 py-2 rounded-lg font-medium transition-colors duration-200"
                                                on:click=move |_| {
                                                    set_viewing_program.set(None);
                                                    set_show_view_modal.set(false);
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