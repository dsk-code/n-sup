# Leptos Development Guide

このガイドはN-Supプロジェクトで使用するLeptos開発パターンと規約を説明します。

## 基本的なコンポーネントパターン

### 基本コンポーネント
```rust
use leptos::prelude::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <div class="my-component">
            <h1>"Hello, Leptos!"</h1>
        </div>
    }
}
```

### プロップ付きコンポーネント
```rust
#[component]
pub fn Button(
    #[prop(into)] text: String,
    #[prop(optional)] disabled: Option<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <button disabled=disabled.unwrap_or(false)>
            {text}
            {children()}
        </button>
    }
}
```

## 状態管理パターン

### シグナルの作成と使用
```rust
#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|c| *c += 1)>
                "Increment"
            </button>
        </div>
    }
}
```

### 複雑な状態管理
```rust
#[derive(Clone, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[component]
pub fn UserManager() -> impl IntoView {
    let (users, set_users) = signal(Vec::<User>::new());
    let (selected_user, set_selected_user) = signal(None::<User>);

    view! {
        <div class="user-manager">
            <UserList users=users set_selected=set_selected_user />
            <UserDetails user=selected_user />
        </div>
    }
}
```

## イベントハンドリング

### 基本的なイベントハンドリング
```rust
#[component]
pub fn InputForm() -> impl IntoView {
    let (value, set_value) = signal(String::new());

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        // フォーム送信処理
    };

    view! {
        <form on:submit=handle_submit>
            <input
                type="text"
                value=value
                on:input=move |ev| set_value.set(event_target_value(&ev))
            />
            <button type="submit">"Submit"</button>
        </form>
    }
}
```

### DOM操作とEffect
```rust
use wasm_bindgen::prelude::*;
use web_sys::window;

#[component]
pub fn ScrollHandler() -> impl IntoView {
    let (is_scrolled, set_is_scrolled) = signal(false);

    Effect::new(move |_| {
        let closure = Closure::wrap(Box::new(move || {
            let window = window().unwrap();
            let scroll_y = window.scroll_y().unwrap_or(0.0);
            set_is_scrolled.set(scroll_y > 100.0);
        }) as Box<dyn Fn()>);

        let window = window().unwrap();
        let _ = window.add_event_listener_with_callback(
            "scroll",
            closure.as_ref().unchecked_ref()
        );
        
        closure.forget();
    });

    view! {
        <header class=move || {
            if is_scrolled.get() { "header scrolled" } else { "header" }
        }>
            "Header"
        </header>
    }
}
```

## ルーティング

### ルーターの設定
```rust
use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/about") view=About />
                <Route path=path!("/users/:id") view=UserDetail />
            </Routes>
        </Router>
    }
}
```

### ナビゲーション
```rust
use leptos_router::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav>
            <A href="/">"Home"</A>
            <A href="/about">"About"</A>
            <A href="/users/1">"User 1"</A>
        </nav>
    }
}
```

## 条件付きレンダリング

### Show コンポーネント
```rust
#[component]
pub fn ConditionalView() -> impl IntoView {
    let (show_content, set_show_content) = signal(false);

    view! {
        <div>
            <button on:click=move |_| set_show_content.update(|s| *s = !*s)>
                "Toggle"
            </button>
            <Show when=move || show_content.get() fallback=|| view! { <p>"Hidden"</p> }>
                <p>"Shown!"</p>
            </Show>
        </div>
    }
}
```

### 動的クラスとスタイル
```rust
#[component]
pub fn DynamicStyling() -> impl IntoView {
    let (active, set_active) = signal(false);

    view! {
        <div
            class=move || {
                if active.get() {
                    "active-class"
                } else {
                    "inactive-class"
                }
            }
            style=move || {
                if active.get() {
                    "color: red;"
                } else {
                    "color: blue;"
                }
            }
        >
            "Dynamic Content"
        </div>
    }
}
```

## リストレンダリング

### For コンポーネント
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

#[component]
pub fn ItemList() -> impl IntoView {
    let (items, set_items) = signal(vec![
        Item { id: 1, name: "Item 1".to_string() },
        Item { id: 2, name: "Item 2".to_string() },
    ]);

    view! {
        <ul>
            <For
                each=move || items.get()
                key=|item| item.id
                children=move |item| {
                    view! {
                        <li>
                            {item.name}
                        </li>
                    }
                }
            />
        </ul>
    }
}
```

## エラーハンドリング

### エラーバウンダリ
```rust
#[component]
pub fn ErrorBoundaryExample() -> impl IntoView {
    view! {
        <ErrorBoundary
            fallback=|errors| view! {
                <div class="error">
                    <h2>"Error occurred:"</h2>
                    <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
            }
        >
            <MaybeErrorComponent />
        </ErrorBoundary>
    }
}
```

## 非同期処理

### Resource の使用
```rust
#[component]
pub fn AsyncData() -> impl IntoView {
    let (user_id, set_user_id) = signal(1);
    
    let user_resource = Resource::new(
        move || user_id.get(),
        |id| async move {
            // 非同期データ取得
            fetch_user(id).await
        }
    );

    view! {
        <div>
            <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                {move || user_resource.get().map(|user| view! {
                    <div>
                        <h2>{user.name}</h2>
                        <p>{user.email}</p>
                    </div>
                })}
            </Suspense>
        </div>
    }
}
```

## ベストプラクティス

### 1. インポートの規約
```rust
// 常に leptos::prelude::* を使用
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
```

### 2. コンポーネント名の規約
- PascalCase を使用: `MyComponent`, `UserProfile`
- ファイル名は snake_case: `my_component.rs`, `user_profile.rs`

### 3. シグナルの命名規約
```rust
// getter/setter ペアの命名
let (count, set_count) = signal(0);
let (user_name, set_user_name) = signal(String::new());
let (is_loading, set_is_loading) = signal(false);
```

### 4. プロップの規約
```rust
#[component]
pub fn MyComponent(
    #[prop(into)] title: String,           // 文字列変換
    #[prop(optional)] disabled: Option<bool>, // オプショナル
    children: Children,                     // 子要素
) -> impl IntoView {
    // ...
}
```

### 5. Effect の使用
```rust
// DOM操作や副作用には Effect を使用
Effect::new(move |_| {
    // 副作用処理
});

// クリーンアップが必要な場合
Effect::new(move |_| {
    let cleanup = setup_something();
    
    // クリーンアップ関数を返す
    move || cleanup()
});
```

### 6. エラーハンドリング
```rust
// Result 型を適切に使用
let handle_action = move |_| {
    match some_operation() {
        Ok(result) => {
            // 成功処理
        }
        Err(error) => {
            // エラー処理
            log::error!("Operation failed: {}", error);
        }
    }
};
```

## パフォーマンス最適化

### 1. メモ化
```rust
// 計算結果をメモ化
let expensive_calculation = Memo::new(move |_| {
    // 重い計算処理
    complex_calculation(input.get())
});
```

### 2. 適切なキーの使用
```rust
// For コンポーネントでは安定したキーを使用
<For
    each=move || items.get()
    key=|item| item.id  // 安定したID
    children=move |item| view! { <ItemView item=item /> }
/>
```

### 3. 不要な再レンダリングの回避
```rust
// 関数を move クロージャ外で定義
let handle_click = move |_| {
    // ハンドラー処理
};

view! {
    <button on:click=handle_click>
        "Click me"
    </button>
}
```

## デバッグとロギング

### ログの使用
```rust
use log::*;

#[component]
pub fn DebugComponent() -> impl IntoView {
    let (value, set_value) = signal(0);

    Effect::new(move |_| {
        debug!("Value changed to: {}", value.get());
    });

    view! {
        <div>
            <p>{value}</p>
            <button on:click=move |_| {
                set_value.update(|v| *v += 1);
                info!("Button clicked, new value: {}", value.get());
            }>
                "Increment"
            </button>
        </div>
    }
}
```

このガイドはN-Supプロジェクトで使用するLeptosの主要パターンをカバーしています。新しいパターンや規約が必要になった場合は、このファイルを更新してください。