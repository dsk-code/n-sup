use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub id: u32,
    pub sender: String,
    pub message: String,
    pub timestamp: String,
    pub message_type: MessageType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageType {
    User,
    System,
    Alert,
}

#[derive(Clone, Debug)]
pub struct ChatRoom {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub member_count: u32,
    pub is_active: bool,
}

#[component]
pub fn Chat() -> impl IntoView {
    let (messages, set_messages) = signal(vec![
        ChatMessage {
            id: 1,
            sender: "田中太郎".to_string(),
            message: "おはようございます！今日の工具の準備はいかがですか？".to_string(),
            timestamp: "09:30".to_string(),
            message_type: MessageType::User,
        },
        ChatMessage {
            id: 2,
            sender: "佐藤花子".to_string(),
            message: "おはようございます。A5052の加工用エンドミルの在庫が少なくなっています。".to_string(),
            timestamp: "09:32".to_string(),
            message_type: MessageType::User,
        },
        ChatMessage {
            id: 3,
            sender: "システム".to_string(),
            message: "工具「エンドミル φ10mm」のメンテナンス期限が近づいています。".to_string(),
            timestamp: "09:35".to_string(),
            message_type: MessageType::Alert,
        },
        ChatMessage {
            id: 4,
            sender: "山田次郎".to_string(),
            message: "NCプログラム「O1001」の最新版をアップロードしました。確認をお願いします。".to_string(),
            timestamp: "09:40".to_string(),
            message_type: MessageType::User,
        },
        ChatMessage {
            id: 5,
            sender: "田中太郎".to_string(),
            message: "承知しました。午後一で確認いたします。".to_string(),
            timestamp: "09:42".to_string(),
            message_type: MessageType::User,
        },
        ChatMessage {
            id: 6,
            sender: "システム".to_string(),
            message: "新しいユーザー「鈴木一郎」がチャットルームに参加しました。".to_string(),
            timestamp: "10:00".to_string(),
            message_type: MessageType::System,
        },
    ]);

    let (chat_rooms, set_chat_rooms) = signal(vec![
        ChatRoom {
            id: 1,
            name: "製造部チーム".to_string(),
            description: "製造部メンバーの連絡用".to_string(),
            member_count: 8,
            is_active: true,
        },
        ChatRoom {
            id: 2,
            name: "工具管理".to_string(),
            description: "工具の在庫・メンテナンス情報共有".to_string(),
            member_count: 5,
            is_active: true,
        },
        ChatRoom {
            id: 3,
            name: "NCプログラム共有".to_string(),
            description: "NCプログラムの更新・共有".to_string(),
            member_count: 12,
            is_active: false,
        },
        ChatRoom {
            id: 4,
            name: "全体連絡".to_string(),
            description: "全社員向けアナウンス".to_string(),
            member_count: 25,
            is_active: false,
        },
    ]);

    let (selected_room, set_selected_room) = signal(1u32);
    let (new_message, set_new_message) = signal(String::new());
    let (show_room_modal, set_show_room_modal) = signal(false);
    let (new_room_name, set_new_room_name) = signal(String::new());
    let (new_room_description, set_new_room_description) = signal(String::new());

    let send_message = move |_: web_sys::MouseEvent| {
        if !new_message.get().is_empty() {
            let message = ChatMessage {
                id: messages.get().len() as u32 + 1,
                sender: "あなた".to_string(),
                message: new_message.get(),
                timestamp: "now".to_string(),
                message_type: MessageType::User,
            };
            set_messages.update(|msgs| msgs.push(message));
            set_new_message.set(String::new());
        }
    };

    let send_message_enter = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !new_message.get().is_empty() {
            let message = ChatMessage {
                id: messages.get().len() as u32 + 1,
                sender: "あなた".to_string(),
                message: new_message.get(),
                timestamp: "now".to_string(),
                message_type: MessageType::User,
            };
            set_messages.update(|msgs| msgs.push(message));
            set_new_message.set(String::new());
        }
    };

    let create_room = move |_| {
        if !new_room_name.get().is_empty() {
            let room = ChatRoom {
                id: chat_rooms.get().len() as u32 + 1,
                name: new_room_name.get(),
                description: new_room_description.get(),
                member_count: 1,
                is_active: true,
            };
            set_chat_rooms.update(|rooms| rooms.push(room));
            set_new_room_name.set(String::new());
            set_new_room_description.set(String::new());
            set_show_room_modal.set(false);
        }
    };

    let message_color = |msg_type: &MessageType| match msg_type {
        MessageType::User => "bg-slate-700/50",
        MessageType::System => "bg-blue-600/20 border-l-4 border-blue-500",
        MessageType::Alert => "bg-red-600/20 border-l-4 border-red-500",
    };

    let current_room_name = move || {
        chat_rooms.get()
            .iter()
            .find(|room| room.id == selected_room.get())
            .map(|room| room.name.clone())
            .unwrap_or_else(|| "チャットルーム".to_string())
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

                <div class="flex flex-col lg:flex-row justify-between items-start lg:items-center gap-4 mb-8">
                    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                        "チャット"
                    </h1>
                    <button 
                        class="w-full lg:w-auto bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 px-4 sm:px-6 py-3 rounded-lg font-semibold transition-all duration-300 transform hover:scale-105 text-sm sm:text-base"
                        on:click=move |_| set_show_room_modal.set(true)
                    >
                        <span class="lg:hidden">"新規ルーム"</span>
                        <span class="hidden lg:inline">"新規チャットルーム作成"</span>
                    </button>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-4 gap-6 h-[600px]">
                    // Chat Rooms Sidebar
                    <div class="lg:col-span-1 bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 p-4">
                        <h2 class="text-lg font-semibold mb-4 text-slate-200">"チャットルーム"</h2>
                        <div class="space-y-2 max-h-[500px] overflow-y-auto">
                            <For
                                each=move || chat_rooms.get()
                                key=|room| room.id
                                children=move |room| {
                                    let room_id = room.id;
                                    let is_selected = move || selected_room.get() == room_id;
                                    view! {
                                        <div 
                                            class={move || format!("p-3 rounded-lg cursor-pointer transition-colors duration-200 {}",
                                                if is_selected() { "bg-blue-600/30 border border-blue-500/50" } else { "bg-slate-700/30 hover:bg-slate-700/50" }
                                            )}
                                            on:click=move |_| set_selected_room.set(room_id)
                                        >
                                            <div class="flex justify-between items-start mb-1">
                                                <h3 class="font-medium text-sm text-white">{room.name.clone()}</h3>
                                                <div class="flex items-center gap-1">
                                                    <span class={format!("w-2 h-2 rounded-full {}", 
                                                        if room.is_active { "bg-green-400" } else { "bg-slate-400" }
                                                    )}></span>
                                                    <span class="text-xs text-slate-400">{room.member_count}</span>
                                                </div>
                                            </div>
                                            <p class="text-xs text-slate-400 line-clamp-2">{room.description.clone()}</p>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>

                    // Chat Area
                    <div class="lg:col-span-3 bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700/50 flex flex-col">
                        // Chat Header
                        <div class="p-4 border-b border-slate-700/50">
                            <h2 class="text-lg font-semibold text-white">{current_room_name}</h2>
                            <p class="text-sm text-slate-400">"チームメンバーとリアルタイムでコミュニケーション"</p>
                        </div>

                        // Messages Area
                        <div class="flex-1 p-4 overflow-y-auto space-y-3 max-h-[400px]">
                            <For
                                each=move || messages.get()
                                key=|message| message.id
                                children=move |message| {
                                    view! {
                                        <div class={format!("p-3 rounded-lg {}", message_color(&message.message_type))}>
                                            <div class="flex justify-between items-start mb-1">
                                                <span class="font-medium text-sm text-blue-300">{message.sender.clone()}</span>
                                                <span class="text-xs text-slate-400">{message.timestamp.clone()}</span>
                                            </div>
                                            <p class="text-slate-200 text-sm leading-relaxed">{message.message.clone()}</p>
                                        </div>
                                    }
                                }
                            />
                        </div>

                        // Message Input
                        <div class="p-4 border-t border-slate-700/50">
                            <div class="flex gap-3">
                                <input 
                                    type="text"
                                    placeholder="メッセージを入力..."
                                    class="flex-1 px-4 py-3 bg-slate-700/50 border border-slate-600/50 rounded-lg text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                    prop:value=move || new_message.get()
                                    on:input=move |ev| set_new_message.set(event_target_value(&ev))
                                    on:keypress=send_message_enter
                                />
                                <button 
                                    class="bg-blue-500 hover:bg-blue-600 px-6 py-3 rounded-lg font-medium transition-colors duration-200 flex items-center gap-2"
                                    on:click=send_message
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
                                    </svg>
                                    <span class="hidden sm:inline">"送信"</span>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Create Room Modal
            <Show when=move || show_room_modal.get()>
                <div class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
                    <div class="bg-slate-800 rounded-xl p-6 w-full max-w-md mx-4 border border-slate-700">
                        <h2 class="text-2xl font-bold mb-6 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                            "新規チャットルーム作成"
                        </h2>
                        
                        <div class="space-y-4">
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"ルーム名"</label>
                                <input 
                                    type="text"
                                    placeholder="例: プロジェクトA討議"
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_room_name.get()
                                    on:input=move |ev| set_new_room_name.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-2">"説明"</label>
                                <textarea 
                                    rows="3"
                                    placeholder="このチャットルームの用途や目的を入力..."
                                    class="w-full px-3 py-2 bg-slate-700 border border-slate-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    prop:value=move || new_room_description.get()
                                    on:input=move |ev| set_new_room_description.set(event_target_value(&ev))
                                ></textarea>
                            </div>
                        </div>
                        
                        <div class="flex gap-3 mt-6">
                            <button 
                                class="flex-1 bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 py-2 rounded-lg font-medium transition-all duration-300"
                                on:click=create_room
                            >
                                "作成"
                            </button>
                            <button 
                                class="flex-1 bg-slate-600 hover:bg-slate-500 py-2 rounded-lg font-medium transition-colors duration-200"
                                on:click=move |_| {
                                    set_new_room_name.set(String::new());
                                    set_new_room_description.set(String::new());
                                    set_show_room_modal.set(false);
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