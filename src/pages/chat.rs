use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub id: u32,
    pub sender: String,
    pub message: String,
    pub timestamp: String,
    pub message_type: MessageType,
    pub priority: MessagePriority,
    pub ai_sentiment: f32,
    pub attachments: Vec<String>,
    pub read_by: Vec<String>,
    pub reactions: Vec<MessageReaction>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageType {
    User,
    System,
    Alert,
    AIAssistant,
    ExecutiveUpdate,
    CriticalAlert,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Critical,
    Presidential,
}

#[derive(Clone, Debug)]
pub struct MessageReaction {
    pub emoji: String,
    pub count: u32,
    pub users: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ChatRoom {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub member_count: u32,
    pub is_active: bool,
    pub room_type: RoomType,
    pub ai_moderation: bool,
    pub encryption_level: EncryptionLevel,
    pub productivity_score: f32,
    pub collaboration_metrics: CollaborationMetrics,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RoomType {
    General,
    Executive,
    ProjectTeam,
    Emergency,
    AIAnalysis,
    ProductionFloor,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EncryptionLevel {
    Standard,
    Enterprise,
    Presidential,
    QuantumSecure,
}

#[derive(Clone, Debug)]
pub struct CollaborationMetrics {
    pub messages_per_hour: f32,
    pub response_rate: f32,
    pub engagement_score: f32,
    pub decision_velocity: f32,
}

#[derive(Clone, Debug)]
pub struct AIInsight {
    pub insight_type: InsightType,
    pub title: String,
    pub description: String,
    pub impact_score: f32,
    pub confidence: f32,
    pub actionable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InsightType {
    CommunicationEfficiency,
    TeamMorale,
    ProductivityTrend,
    DecisionBottleneck,
    CollaborationGap,
}

#[derive(Clone, Debug)]
pub struct ChatAnalytics {
    pub total_messages: u32,
    pub active_users: u32,
    pub average_response_time: f32,
    pub sentiment_score: f32,
    pub productivity_index: f32,
    pub collaboration_efficiency: f32,
    pub ai_insights: Vec<AIInsight>,
}

#[component]
pub fn Chat() -> impl IntoView {
    let (messages, set_messages) = signal(vec![
        ChatMessage {
            id: 1,
            sender: "Presidential AI Assistant".to_string(),
            message: "🚀 おはようございます！今日の製造効率は97.3%で推移中。工具最適化により34%のコスト削減を達成しています。".to_string(),
            timestamp: "09:30".to_string(),
            message_type: MessageType::AIAssistant,
            priority: MessagePriority::Presidential,
            ai_sentiment: 0.95,
            attachments: vec!["efficiency_report.pdf".to_string()],
            read_by: vec!["田中太郎".to_string(), "佐藤花子".to_string()],
            reactions: vec![MessageReaction { emoji: "💼".to_string(), count: 5, users: vec!["田中太郎".to_string()] }],
        },
        ChatMessage {
            id: 2,
            sender: "田中太郎".to_string(),
            message: "素晴らしい結果ですね！A5052の加工でエンドミルの性能向上が顕著に現れています。".to_string(),
            timestamp: "09:32".to_string(),
            message_type: MessageType::User,
            priority: MessagePriority::High,
            ai_sentiment: 0.87,
            attachments: vec![],
            read_by: vec!["佐藤花子".to_string(), "山田次郎".to_string()],
            reactions: vec![MessageReaction { emoji: "👍".to_string(), count: 3, users: vec!["佐藤花子".to_string()] }],
        },
        ChatMessage {
            id: 3,
            sender: "Quantum Alert System".to_string(),
            message: "⚡ 予知保全AI：工具「エンドミル φ10mm」の交換タイミングを97.8%の精度で予測。最適な交換時期は明日14:30です。".to_string(),
            timestamp: "09:35".to_string(),
            message_type: MessageType::CriticalAlert,
            priority: MessagePriority::Critical,
            ai_sentiment: 0.65,
            attachments: vec!["predictive_maintenance.json".to_string()],
            read_by: vec!["田中太郎".to_string()],
            reactions: vec![MessageReaction { emoji: "🔧".to_string(), count: 2, users: vec!["山田次郎".to_string()] }],
        },
        ChatMessage {
            id: 4,
            sender: "山田次郎".to_string(),
            message: "🎯 AIが最適化したNCプログラム「O1001 v3.0」をデプロイ完了。実行時間23%短縮、品質スコア99.1%を実現しました！".to_string(),
            timestamp: "09:40".to_string(),
            message_type: MessageType::User,
            priority: MessagePriority::High,
            ai_sentiment: 0.92,
            attachments: vec!["nc_program_v3.nc".to_string(), "performance_metrics.xlsx".to_string()],
            read_by: vec!["田中太郎".to_string(), "佐藤花子".to_string()],
            reactions: vec![MessageReaction { emoji: "🚀".to_string(), count: 4, users: vec!["田中太郎".to_string(), "佐藤花子".to_string()] }],
        },
        ChatMessage {
            id: 5,
            sender: "佐藤花子".to_string(),
            message: "💰 今月のコスト削減実績：4,200万円達成！目標を125%上回る驚異的な成果です。".to_string(),
            timestamp: "09:42".to_string(),
            message_type: MessageType::ExecutiveUpdate,
            priority: MessagePriority::Presidential,
            ai_sentiment: 0.98,
            attachments: vec!["cost_savings_report.pdf".to_string()],
            read_by: vec!["田中太郎".to_string(), "山田次郎".to_string()],
            reactions: vec![MessageReaction { emoji: "💎".to_string(), count: 6, users: vec!["田中太郎".to_string(), "山田次郎".to_string()] }],
        },
        ChatMessage {
            id: 6,
            sender: "Presidential Communication System".to_string(),
            message: "🌟 新メンバー「鈴木一郎（量子AI専門家）」が参加。チームの集合知能指数が15.7%向上しました。".to_string(),
            timestamp: "10:00".to_string(),
            message_type: MessageType::System,
            priority: MessagePriority::High,
            ai_sentiment: 0.85,
            attachments: vec!["team_intelligence_metrics.json".to_string()],
            read_by: vec![],
            reactions: vec![MessageReaction { emoji: "🎉".to_string(), count: 8, users: vec!["田中太郎".to_string(), "佐藤花子".to_string(), "山田次郎".to_string()] }],
        },
    ]);

    let (chat_rooms, set_chat_rooms) = signal(vec![
        ChatRoom {
            id: 1,
            name: "🏆 Presidential Command Center".to_string(),
            description: "最高経営陣による戦略的意思決定とリアルタイム業績監視".to_string(),
            member_count: 8,
            is_active: true,
            room_type: RoomType::Executive,
            ai_moderation: true,
            encryption_level: EncryptionLevel::Presidential,
            productivity_score: 97.3,
            collaboration_metrics: CollaborationMetrics {
                messages_per_hour: 15.2,
                response_rate: 98.7,
                engagement_score: 95.1,
                decision_velocity: 87.4,
            },
        },
        ChatRoom {
            id: 2,
            name: "🤖 AI Quantum Analytics Hub".to_string(),
            description: "次世代AI分析と量子コンピューティングによる製造最適化".to_string(),
            member_count: 12,
            is_active: true,
            room_type: RoomType::AIAnalysis,
            ai_moderation: true,
            encryption_level: EncryptionLevel::QuantumSecure,
            productivity_score: 99.8,
            collaboration_metrics: CollaborationMetrics {
                messages_per_hour: 24.7,
                response_rate: 99.2,
                engagement_score: 96.8,
                decision_velocity: 92.3,
            },
        },
        ChatRoom {
            id: 3,
            name: "⚡ Production Floor Intelligence".to_string(),
            description: "製造現場のリアルタイム監視とAI予測分析".to_string(),
            member_count: 18,
            is_active: true,
            room_type: RoomType::ProductionFloor,
            ai_moderation: true,
            encryption_level: EncryptionLevel::Enterprise,
            productivity_score: 94.6,
            collaboration_metrics: CollaborationMetrics {
                messages_per_hour: 32.1,
                response_rate: 95.8,
                engagement_score: 91.2,
                decision_velocity: 89.7,
            },
        },
        ChatRoom {
            id: 4,
            name: "🚨 Emergency Response Center".to_string(),
            description: "緊急事態対応と危機管理のための専用チャンネル".to_string(),
            member_count: 25,
            is_active: false,
            room_type: RoomType::Emergency,
            ai_moderation: true,
            encryption_level: EncryptionLevel::Presidential,
            productivity_score: 88.9,
            collaboration_metrics: CollaborationMetrics {
                messages_per_hour: 8.3,
                response_rate: 99.9,
                engagement_score: 97.5,
                decision_velocity: 95.8,
            },
        },
        ChatRoom {
            id: 5,
            name: "💎 Executive Innovation Lab".to_string(),
            description: "次世代技術とイノベーション戦略の討議".to_string(),
            member_count: 6,
            is_active: true,
            room_type: RoomType::Executive,
            ai_moderation: true,
            encryption_level: EncryptionLevel::Presidential,
            productivity_score: 96.4,
            collaboration_metrics: CollaborationMetrics {
                messages_per_hour: 11.8,
                response_rate: 97.3,
                engagement_score: 94.7,
                decision_velocity: 91.2,
            },
        },
    ]);

    let (analytics, _set_analytics) = signal(ChatAnalytics {
        total_messages: 2847,
        active_users: 47,
        average_response_time: 2.3,
        sentiment_score: 87.4,
        productivity_index: 94.7,
        collaboration_efficiency: 92.8,
        ai_insights: vec![
            AIInsight {
                insight_type: InsightType::CommunicationEfficiency,
                title: "コミュニケーション効率の最適化".to_string(),
                description: "チーム間の情報共有速度が23%向上。AI支援により意思決定の迅速化を実現。".to_string(),
                impact_score: 8.7,
                confidence: 94.2,
                actionable: true,
            },
            AIInsight {
                insight_type: InsightType::TeamMorale,
                title: "チームモラルの向上".to_string(),
                description: "ポジティブな感情分析が85%を記録。プロジェクト成功率との強い相関を確認。".to_string(),
                impact_score: 9.2,
                confidence: 89.7,
                actionable: true,
            },
            AIInsight {
                insight_type: InsightType::ProductivityTrend,
                title: "生産性向上トレンド".to_string(),
                description: "過去30日間で生産性指標が15.3%上昇。AIアシスタントの活用効果が顕著。".to_string(),
                impact_score: 9.8,
                confidence: 96.1,
                actionable: true,
            },
        ],
    });

    let (show_analytics, set_show_analytics) = signal(false);

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
                priority: MessagePriority::Normal,
                ai_sentiment: 0.75,
                attachments: vec![],
                read_by: vec![],
                reactions: vec![],
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
                priority: MessagePriority::Normal,
                ai_sentiment: 0.75,
                attachments: vec![],
                read_by: vec![],
                reactions: vec![],
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
                room_type: RoomType::ProjectTeam,
                ai_moderation: true,
                encryption_level: EncryptionLevel::Enterprise,
                productivity_score: 85.0,
                collaboration_metrics: CollaborationMetrics {
                    messages_per_hour: 12.0,
                    response_rate: 90.0,
                    engagement_score: 85.0,
                    decision_velocity: 80.0,
                },
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
        MessageType::AIAssistant => "bg-gradient-to-r from-purple-900/30 to-blue-900/30 border-l-4 border-purple-500",
        MessageType::ExecutiveUpdate => "bg-gradient-to-r from-yellow-900/30 to-orange-900/30 border-l-4 border-yellow-500",
        MessageType::CriticalAlert => "bg-gradient-to-r from-red-900/40 to-pink-900/40 border-l-4 border-red-400 animate-pulse",
    };

    let priority_indicator = |priority: &MessagePriority| match priority {
        MessagePriority::Low => "🔵",
        MessagePriority::Normal => "⚪",
        MessagePriority::High => "🟡",
        MessagePriority::Critical => "🔴",
        MessagePriority::Presidential => "👑",
    };

    let room_type_icon = |room_type: &RoomType| match room_type {
        RoomType::General => "💬",
        RoomType::Executive => "🏢",
        RoomType::ProjectTeam => "👥",
        RoomType::Emergency => "🚨",
        RoomType::AIAnalysis => "🤖",
        RoomType::ProductionFloor => "🏭",
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