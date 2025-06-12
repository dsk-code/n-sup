use leptos::prelude::*;
use crate::components::BackToHome;

#[derive(Clone, Debug)]
pub struct Employee {
    pub id: u32,
    pub name: String,
    pub employee_id: String,
    pub department: String,
    pub position: String,
    pub email: String,
    pub phone: String,
    pub hire_date: String,
    pub status: EmployeeStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EmployeeStatus {
    Active,
    OnLeave,
    Inactive,
}

#[component]
pub fn EmployeeManagement() -> impl IntoView {
    let (employees, set_employees) = signal(vec![
        Employee {
            id: 1,
            name: "田中太郎".to_string(),
            employee_id: "EMP001".to_string(),
            department: "製造部".to_string(),
            position: "主任".to_string(),
            email: "tanaka@nsup.com".to_string(),
            phone: "090-1234-5678".to_string(),
            hire_date: "2020-04-01".to_string(),
            status: EmployeeStatus::Active,
        },
        Employee {
            id: 2,
            name: "佐藤花子".to_string(),
            employee_id: "EMP002".to_string(),
            department: "品質管理部".to_string(),
            position: "検査員".to_string(),
            email: "sato@nsup.com".to_string(),
            phone: "090-9876-5432".to_string(),
            hire_date: "2021-07-15".to_string(),
            status: EmployeeStatus::Active,
        },
        Employee {
            id: 3,
            name: "山田次郎".to_string(),
            employee_id: "EMP003".to_string(),
            department: "技術部".to_string(),
            position: "エンジニア".to_string(),
            email: "yamada@nsup.com".to_string(),
            phone: "090-1111-2222".to_string(),
            hire_date: "2019-10-01".to_string(),
            status: EmployeeStatus::OnLeave,
        },
    ]);

    let (show_add_modal, set_show_add_modal) = signal(false);
    let (show_edit_modal, set_show_edit_modal) = signal(false);
    let (editing_employee, set_editing_employee) = signal(None::<Employee>);
    
    let (new_name, set_new_name) = signal(String::new());
    let (new_employee_id, set_new_employee_id) = signal(String::new());
    let (new_department, set_new_department) = signal(String::new());
    let (new_position, set_new_position) = signal(String::new());
    let (new_email, set_new_email) = signal(String::new());
    let (new_phone, set_new_phone) = signal(String::new());

    let clear_form = move || {
        set_new_name.set(String::new());
        set_new_employee_id.set(String::new());
        set_new_department.set(String::new());
        set_new_position.set(String::new());
        set_new_email.set(String::new());
        set_new_phone.set(String::new());
    };

    let add_employee = move |_| {
        if !new_name.get().is_empty() && !new_employee_id.get().is_empty() {
            let new_employee = Employee {
                id: employees.get().len() as u32 + 1,
                name: new_name.get(),
                employee_id: new_employee_id.get(),
                department: new_department.get(),
                position: new_position.get(),
                email: new_email.get(),
                phone: new_phone.get(),
                hire_date: "2024-06-12".to_string(),
                status: EmployeeStatus::Active,
            };
            set_employees.update(|employees| employees.push(new_employee));
            clear_form();
            set_show_add_modal.set(false);
        }
    };

    let edit_employee = move |employee: Employee| {
        set_editing_employee.set(Some(employee.clone()));
        set_new_name.set(employee.name);
        set_new_employee_id.set(employee.employee_id);
        set_new_department.set(employee.department);
        set_new_position.set(employee.position);
        set_new_email.set(employee.email);
        set_new_phone.set(employee.phone);
        set_show_edit_modal.set(true);
    };

    let update_employee = move |_| {
        if let Some(editing) = editing_employee.get() {
            set_employees.update(|employees| {
                if let Some(emp) = employees.iter_mut().find(|e| e.id == editing.id) {
                    emp.name = new_name.get();
                    emp.employee_id = new_employee_id.get();
                    emp.department = new_department.get();
                    emp.position = new_position.get();
                    emp.email = new_email.get();
                    emp.phone = new_phone.get();
                }
            });
            clear_form();
            set_editing_employee.set(None);
            set_show_edit_modal.set(false);
        }
    };

    let delete_employee = move |id: u32| {
        set_employees.update(|employees| {
            employees.retain(|employee| employee.id != id);
        });
    };

    let status_color = |status: &EmployeeStatus| match status {
        EmployeeStatus::Active => "bg-green-100 text-green-800",
        EmployeeStatus::OnLeave => "bg-yellow-100 text-yellow-800",
        EmployeeStatus::Inactive => "bg-red-100 text-red-800",
    };

    let status_text = |status: &EmployeeStatus| match status {
        EmployeeStatus::Active => "在職",
        EmployeeStatus::OnLeave => "休職中",
        EmployeeStatus::Inactive => "退職",
    };

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 to-slate-800 text-white">
            <div class="container mx-auto px-4 py-8">
                <BackToHome />
                <div class="flex justify-between items-center mb-8">
                    <h1 class="text-4xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                        "従業員管理"
                    </h1>
                    <button 
                        class="bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 px-6 py-3 rounded-lg font-semibold transition-all duration-300 transform hover:scale-105"
                        on:click=move |_| set_show_add_modal.set(true)
                    >
                        "新規従業員追加"
                    </button>
                </div>

                <div class="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 shadow-xl overflow-hidden">
                    <div class="overflow-x-auto">
                        <table class="w-full">
                            <thead class="bg-slate-700/50">
                                <tr>
                                    <th class="px-6 py-4 text-left text-sm font-semibold text-slate-300">"氏名"</th>
                                    <th class="px-6 py-4 text-left text-sm font-semibold text-slate-300">"従業員ID"</th>
                                    <th class="px-6 py-4 text-left text-sm font-semibold text-slate-300">"部署"</th>
                                    <th class="px-6 py-4 text-left text-sm font-semibold text-slate-300">"役職"</th>
                                    <th class="px-6 py-4 text-left text-sm font-semibold text-slate-300">"メールアドレス"</th>
                                    <th class="px-6 py-4 text-left text-sm font-semibold text-slate-300">"電話番号"</th>
                                    <th class="px-6 py-4 text-left text-sm font-semibold text-slate-300">"入社日"</th>
                                    <th class="px-6 py-4 text-left text-sm font-semibold text-slate-300">"ステータス"</th>
                                    <th class="px-6 py-4 text-left text-sm font-semibold text-slate-300">"操作"</th>
                                </tr>
                            </thead>
                            <tbody class="divide-y divide-slate-700/50">
                                <For
                                    each=move || employees.get()
                                    key=|employee| employee.id
                                    children=move |employee| {
                                        let employee_id = employee.id;
                                        let employee_for_edit = employee.clone();
                                        view! {
                                            <tr class="hover:bg-slate-700/30 transition-colors duration-200">
                                                <td class="px-6 py-4 font-medium">{employee.name.clone()}</td>
                                                <td class="px-6 py-4 text-slate-300">{employee.employee_id.clone()}</td>
                                                <td class="px-6 py-4 text-slate-300">{employee.department.clone()}</td>
                                                <td class="px-6 py-4 text-slate-300">{employee.position.clone()}</td>
                                                <td class="px-6 py-4 text-slate-300">{employee.email.clone()}</td>
                                                <td class="px-6 py-4 text-slate-300">{employee.phone.clone()}</td>
                                                <td class="px-6 py-4 text-slate-300">{employee.hire_date.clone()}</td>
                                                <td class="px-6 py-4">
                                                    <span class={format!("px-3 py-1 rounded-full text-xs font-medium {}", status_color(&employee.status))}>
                                                        {status_text(&employee.status)}
                                                    </span>
                                                </td>
                                                <td class="px-6 py-4">
                                                    <div class="flex gap-2">
                                                        <button 
                                                            class="bg-blue-500 hover:bg-blue-600 px-3 py-1 rounded text-sm font-medium transition-colors duration-200"
                                                            on:click=move |_| edit_employee(employee_for_edit.clone())
                                                        >
                                                            "編集"
                                                        </button>
                                                        <button 
                                                            class="bg-red-500 hover:bg-red-600 px-3 py-1 rounded text-sm font-medium transition-colors duration-200"
                                                            on:click=move |_| delete_employee(employee_id)
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
            </div>

            // Add Employee Modal
            <Show when=move || show_add_modal.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-2xl mx-4 border border-slate-700 max-h-[90vh] overflow-y-auto">
                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                            "新規従業員追加"
                        </h2>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"氏名"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_name.get()
                                    on:input=move |ev| set_new_name.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"従業員ID"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_employee_id.get()
                                    on:input=move |ev| set_new_employee_id.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"部署"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_department.get()
                                    on:input=move |ev| set_new_department.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"役職"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_position.get()
                                    on:input=move |ev| set_new_position.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"メールアドレス"</label>
                                <input 
                                    type="email"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_email.get()
                                    on:input=move |ev| set_new_email.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"電話番号"</label>
                                <input 
                                    type="tel"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_phone.get()
                                    on:input=move |ev| set_new_phone.set(event_target_value(&ev))
                                />
                            </div>
                        </div>
                        
                        <div class="flex gap-3 mt-6">
                            <button 
                                class="flex-1 bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 py-2 rounded-lg font-medium transition-all duration-300"
                                on:click=add_employee
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

            // Edit Employee Modal
            <Show when=move || show_edit_modal.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-2xl mx-4 border border-slate-700 max-h-[90vh] overflow-y-auto">
                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                            "従業員情報編集"
                        </h2>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"氏名"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_name.get()
                                    on:input=move |ev| set_new_name.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"従業員ID"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_employee_id.get()
                                    on:input=move |ev| set_new_employee_id.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"部署"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_department.get()
                                    on:input=move |ev| set_new_department.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"役職"</label>
                                <input 
                                    type="text"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_position.get()
                                    on:input=move |ev| set_new_position.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"メールアドレス"</label>
                                <input 
                                    type="email"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_email.get()
                                    on:input=move |ev| set_new_email.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"電話番号"</label>
                                <input 
                                    type="tel"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_phone.get()
                                    on:input=move |ev| set_new_phone.set(event_target_value(&ev))
                                />
                            </div>
                        </div>
                        
                        <div class="flex gap-3 mt-6">
                            <button 
                                class="flex-1 bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 py-2 rounded-lg font-medium transition-all duration-300"
                                on:click=update_employee
                            >
                                "更新"
                            </button>
                            <button 
                                class="flex-1 bg-slate-600 hover:bg-slate-500 py-2 rounded-lg font-medium transition-colors duration-200"
                                on:click=move |_| {
                                    clear_form();
                                    set_editing_employee.set(None);
                                    set_show_edit_modal.set(false);
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