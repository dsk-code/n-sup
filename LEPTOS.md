├── .github
    ├── FUNDING.yml
    ├── ISSUE_TEMPLATE
    │   ├── bug_report.md
    │   ├── config.yml
    │   └── feature_request.md
    └── workflows
    │   ├── autofix.yml
    │   ├── ci.yml
    │   ├── get-example-changed.yml
    │   ├── get-examples-matrix.yml
    │   ├── get-leptos-changed.yml
    │   ├── get-leptos-matrix.yml
    │   ├── publish-book.yml
    │   └── run-cargo-make-task.yml
├── .gitignore
├── ARCHITECTURE.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── Makefile.toml
├── README.md
├── SECURITY.md
├── TODO.md
├── any_error
    ├── Cargo.toml
    ├── Makefile.toml
    ├── README.md
    └── src
    │   └── lib.rs
├── any_spawner
    ├── Cargo.toml
    ├── Makefile.toml
    ├── README.md
    ├── src
    │   └── lib.rs
    └── tests
    │   ├── already_set_error.rs
    │   ├── async_executor.rs
    │   ├── custom_executor.rs
    │   ├── custom_runtime.rs
    │   ├── executor_tick.rs
    │   ├── futures_executor.rs
    │   ├── futures_runtime.rs
    │   ├── glib.rs
    │   ├── local_custom_executor.rs
    │   ├── multiple_tasks.rs
    │   ├── tokio_executor.rs
    │   └── wasm_bindgen_tests.rs
├── benchmarks
    ├── Cargo.toml
    └── src
    │   ├── lib.rs
    │   ├── reactive.rs
    │   ├── ssr.rs
    │   └── todomvc
    │       ├── leptos.rs
    │       ├── mod.rs
    │       ├── sycamore.rs
    │       ├── tachys.rs
    │       ├── tera.rs
    │       └── yew.rs
├── cargo-make
    ├── lint.toml
    ├── main.toml
    ├── test.toml
    └── wasm-test.toml
├── const_str_slice_concat
    ├── Cargo.toml
    ├── Makefile.toml
    └── src
    │   └── lib.rs
├── docs
    ├── COMMON_BUGS.md
    ├── book
    │   ├── .gitignore
    │   ├── README.md
    │   ├── book.toml
    │   ├── mdbook-admonish.css
    │   └── src
    │   │   ├── 01_introduction.md
    │   │   ├── 15_global_state.md
    │   │   ├── SUMMARY.md
    │   │   ├── appendix_reactive_graph.md
    │   │   ├── async
    │   │       ├── 10_resources.md
    │   │       ├── 11_suspense.md
    │   │       ├── 12_transition.md
    │   │       ├── 13_actions.md
    │   │       └── README.md
    │   │   ├── csr_wrapping_up.md
    │   │   ├── deployment
    │   │       ├── README.md
    │   │       └── binary_size.md
    │   │   ├── getting_started
    │   │       ├── README.md
    │   │       ├── community_crates.md
    │   │       └── leptos_dx.md
    │   │   ├── interlude_projecting_children.md
    │   │   ├── interlude_styling.md
    │   │   ├── islands.md
    │   │   ├── metadata.md
    │   │   ├── progressive_enhancement
    │   │       ├── README.md
    │   │       └── action_form.md
    │   │   ├── reactivity
    │   │       ├── 14_create_effect.md
    │   │       ├── README.md
    │   │       ├── interlude_functions.md
    │   │       └── working_with_signals.md
    │   │   ├── router
    │   │       ├── 16_routes.md
    │   │       ├── 17_nested_routing.md
    │   │       ├── 18_params_and_queries.md
    │   │       ├── 19_a.md
    │   │       ├── 20_form.md
    │   │       └── README.md
    │   │   ├── server
    │   │       ├── 25_server_functions.md
    │   │       ├── 26_extractors.md
    │   │       ├── 27_response.md
    │   │       └── README.md
    │   │   ├── ssr
    │   │       ├── 21_cargo_leptos.md
    │   │       ├── 22_life_cycle.md
    │   │       ├── 23_ssr_modes.md
    │   │       ├── 24_hydration_bugs.md
    │   │       └── README.md
    │   │   ├── testing.md
    │   │   └── view
    │   │       ├── 01_basic_component.md
    │   │       ├── 02_dynamic_attributes.md
    │   │       ├── 03_components.md
    │   │       ├── 04_iteration.md
    │   │       ├── 04b_iteration.md
    │   │       ├── 05_forms.md
    │   │       ├── 06_control_flow.md
    │   │       ├── 07_errors.md
    │   │       ├── 08_parent_child.md
    │   │       ├── 09_component_children.md
    │   │       ├── README.md
    │   │       └── builder.md
    ├── book_ru
    │   ├── .gitignore
    │   ├── README.md
    │   ├── book.toml
    │   ├── mdbook-admonish.css
    │   └── src
    │   │   ├── 01_introduction.md
    │   │   ├── 15_global_state.md
    │   │   ├── SUMMARY.md
    │   │   ├── appendix_reactive_graph.md
    │   │   ├── async
    │   │       ├── 10_resources.md
    │   │       ├── 11_suspense.md
    │   │       ├── 12_transition.md
    │   │       ├── 13_actions.md
    │   │       └── README.md
    │   │   ├── csr_wrapping_up.md
    │   │   ├── deployment
    │   │       ├── README.md
    │   │       └── binary_size.md
    │   │   ├── getting_started
    │   │       ├── README.md
    │   │       ├── community_crates.md
    │   │       └── leptos_dx.md
    │   │   ├── interlude_projecting_children.md
    │   │   ├── interlude_styling.md
    │   │   ├── islands.md
    │   │   ├── metadata.md
    │   │   ├── progressive_enhancement
    │   │       ├── README.md
    │   │       └── action_form.md
    │   │   ├── reactivity
    │   │       ├── 14_create_effect.md
    │   │       ├── README.md
    │   │       ├── interlude_functions.md
    │   │       └── working_with_signals.md
    │   │   ├── router
    │   │       ├── 16_routes.md
    │   │       ├── 17_nested_routing.md
    │   │       ├── 18_params_and_queries.md
    │   │       ├── 19_a.md
    │   │       ├── 20_form.md
    │   │       └── README.md
    │   │   ├── server
    │   │       ├── 25_server_functions.md
    │   │       ├── 26_extractors.md
    │   │       ├── 27_response.md
    │   │       └── README.md
    │   │   ├── ssr
    │   │       ├── 21_cargo_leptos.md
    │   │       ├── 22_life_cycle.md
    │   │       ├── 23_ssr_modes.md
    │   │       ├── 24_hydration_bugs.md
    │   │       └── README.md
    │   │   ├── testing.md
    │   │   └── view
    │   │       ├── 01_basic_component.md
    │   │       ├── 02_dynamic_attributes.md
    │   │       ├── 03_components.md
    │   │       ├── 04_iteration.md
    │   │       ├── 04b_iteration.md
    │   │       ├── 05_forms.md
    │   │       ├── 06_control_flow.md
    │   │       ├── 07_errors.md
    │   │       ├── 08_parent_child.md
    │   │       ├── 09_component_children.md
    │   │       ├── README.md
    │   │       └── builder.md
    ├── logos
    │   ├── .gitignore
    │   ├── Leptos_logo_RGB.png
    │   ├── Leptos_logo_RGB.svg
    │   ├── Leptos_logo_Solid_Black.svg
    │   ├── Leptos_logo_Solid_White.svg
    │   ├── Leptos_logo_abbreviation__circle_RGB.png
    │   ├── Leptos_logo_abbreviation__circle_RGB.svg
    │   ├── Leptos_logo_abbreviation__square_RGB.png
    │   ├── Leptos_logo_abbreviation__square_RGB.svg
    │   └── Leptos_logo_pref_dark_RGB.svg
    └── video
    │   ├── async.mov
    │   ├── in-order.mov
    │   └── out-of-order.mov
├── either_of
    ├── Cargo.toml
    ├── Makefile.toml
    ├── README.md
    └── src
    │   └── lib.rs
├── examples
    ├── Makefile.toml
    ├── README.md
    ├── SSR_NOTES.md
    ├── action-form-error-handling
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── src
    │   │   ├── app.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── style
    │   │   └── main.scss
    ├── axum_js_ssr
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── assets
    │   │   └── favicon.ico
    │   ├── node_modules
    │   │   └── @highlightjs
    │   │   │   └── cdn-assets
    │   │   │       ├── LICENSE
    │   │   │       ├── README.md
    │   │   │       ├── es
    │   │   │           └── highlight.min.js
    │   │   │       ├── highlight.min.js
    │   │   │       ├── package.json
    │   │   │       └── styles
    │   │   │           ├── github-dark.min.css
    │   │   │           └── github.min.css
    │   ├── package.json
    │   ├── src
    │   │   ├── api.rs
    │   │   ├── app.rs
    │   │   ├── consts.rs
    │   │   ├── hljs.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── style
    │   │   └── main.scss
    ├── cargo-make
    │   ├── cargo-leptos-compress.toml
    │   ├── cargo-leptos-test.toml
    │   ├── cargo-leptos-webdriver-test.toml
    │   ├── cargo-leptos.toml
    │   ├── clean.toml
    │   ├── client-process.toml
    │   ├── compile.toml
    │   ├── deno-build.toml
    │   ├── lint.toml
    │   ├── main.toml
    │   ├── node.toml
    │   ├── playwright-test.toml
    │   ├── playwright-trunk-test.toml
    │   ├── playwright.toml
    │   ├── process.toml
    │   ├── scripts
    │   │   └── web-report.sh
    │   ├── server-process.toml
    │   ├── trunk_server.toml
    │   ├── wasm-test.toml
    │   └── webdriver.toml
    ├── counter
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── tests
    │   │   └── web.rs
    ├── counter_isomorphic
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── public
    │   │   └── favicon.ico
    │   └── src
    │   │   ├── counters.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── counter_url_query
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── counter_without_macros
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── tests
    │   │   ├── business.rs
    │   │   └── web.rs
    ├── counters
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── e2e
    │   │   ├── .gitignore
    │   │   ├── package-lock.json
    │   │   ├── package.json
    │   │   ├── playwright.config.ts
    │   │   └── tests
    │   │   │   ├── add_1k_counters.spec.ts
    │   │   │   ├── add_counter.spec.ts
    │   │   │   ├── clear_counters.spec.ts
    │   │   │   ├── decrement_count.spec.ts
    │   │   │   ├── enter_count.spec.ts
    │   │   │   ├── fixtures
    │   │   │       └── counters_page.ts
    │   │   │   ├── increment_count.spec.ts
    │   │   │   ├── remove_counter.spec.ts
    │   │   │   └── view_counters.spec.ts
    │   ├── index.html
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── tests
    │   │   └── web.rs
    ├── directives
    │   ├── .cargo
    │   │   └── config.toml
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── tests
    │   │   └── web.rs
    ├── error_boundary
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── e2e
    │   │   ├── .gitignore
    │   │   ├── package-lock.json
    │   │   ├── package.json
    │   │   ├── playwright.config.ts
    │   │   └── tests
    │   │   │   ├── clear_number.spec.ts
    │   │   │   ├── click_down_arrow.spec.ts
    │   │   │   ├── click_up_arrow.spec.ts
    │   │   │   ├── fixtures
    │   │   │       └── home_page.ts
    │   │   │   ├── open_app.spec.ts
    │   │   │   └── type_number.spec.ts
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── errors_axum
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── error_template.rs
    │   │   ├── errors.rs
    │   │   ├── landing.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── style.css
    ├── fetch
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── hackernews
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── api.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   ├── routes.rs
    │   │   └── routes
    │   │   │   ├── nav.rs
    │   │   │   ├── stories.rs
    │   │   │   ├── story.rs
    │   │   │   └── users.rs
    │   └── style.css
    ├── hackernews_axum
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── api.rs
    │   │   ├── error_template.rs
    │   │   ├── handlers.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   ├── routes.rs
    │   │   └── routes
    │   │   │   ├── nav.rs
    │   │   │   ├── stories.rs
    │   │   │   ├── story.rs
    │   │   │   └── users.rs
    │   └── style.css
    ├── hackernews_islands_axum
    │   ├── .cargo
    │   │   └── config.wasm.toml
    │   ├── Cargo.toml
    │   ├── Dockerfile
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── api.rs
    │   │   ├── fallback.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   ├── routes.rs
    │   │   └── routes
    │   │   │   ├── nav.rs
    │   │   │   ├── stories.rs
    │   │   │   ├── story.rs
    │   │   │   └── users.rs
    │   └── style.css
    ├── hackernews_js_fetch
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── deno.jsonc
    │   ├── deno.lock
    │   ├── public
    │   │   ├── favicon.ico
    │   │   └── style.css
    │   ├── run.ts
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── api.rs
    │   │   ├── lib.rs
    │   │   ├── routes.rs
    │   │   └── routes
    │   │       ├── nav.rs
    │   │       ├── stories.rs
    │   │       ├── story.rs
    │   │       └── users.rs
    ├── islands
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── app.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── style.css
    ├── islands_router
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── mock_data.json
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── app.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── style.css
    ├── js-framework-benchmark
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── tests
    │   │   └── web.rs
    ├── parent_child
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── portal
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── tests
    │   │   └── web.rs
    ├── router
    │   ├── .cargo
    │   │   └── config.toml
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── e2e
    │   │   ├── package-lock.json
    │   │   ├── package.json
    │   │   ├── playwright.config.ts
    │   │   └── tests
    │   │   │   └── router.spec.ts
    │   ├── index.html
    │   ├── package.json
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── api.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── style.css
    ├── server_fns_axum
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── app.rs
    │   │   ├── error_template.rs
    │   │   ├── errors.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   └── middleware.rs
    │   ├── style.css
    │   └── watched_files
    │   │   └── .gitkeep
    ├── slots
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── spread
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   └── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── ssr_modes
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── assets
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── app.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── style
    │   │   └── main.scss
    ├── ssr_modes_axum
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── assets
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── app.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── style
    │   │   └── main.scss
    ├── static_routing
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── assets
    │   │   └── favicon.ico
    │   ├── posts
    │   │   ├── post1.md
    │   │   ├── post2.md
    │   │   ├── post3.md
    │   │   └── post4.md
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── app.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    │   └── style
    │   │   └── main.scss
    ├── stores
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── suspense_tests
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── e2e
    │   │   ├── Cargo.toml
    │   │   ├── Makefile.toml
    │   │   ├── README.md
    │   │   ├── features
    │   │   │   ├── check_aria_current.feature
    │   │   │   ├── check_instrumented.feature
    │   │   │   ├── check_instrumented_issue_3719.feature
    │   │   │   ├── check_instrumented_suspense_resource.feature
    │   │   │   ├── click_inside_component_count.feature
    │   │   │   ├── click_nested_count.feature
    │   │   │   ├── click_nested_inside_count.feature
    │   │   │   ├── click_no_resources_count_1.feature
    │   │   │   ├── click_no_resources_count_2.feature
    │   │   │   ├── click_parallel_count_1.feature
    │   │   │   ├── click_parallel_count_2.feature
    │   │   │   ├── click_single_count.feature
    │   │   │   ├── open_app.feature
    │   │   │   ├── view_inside_component.feature
    │   │   │   ├── view_nested.feature
    │   │   │   ├── view_nested_inside.feature
    │   │   │   ├── view_no_resources.feature
    │   │   │   ├── view_parallel.feature
    │   │   │   └── view_single.feature
    │   │   └── tests
    │   │   │   ├── app_suite.rs
    │   │   │   └── fixtures
    │   │   │       ├── action.rs
    │   │   │       ├── check.rs
    │   │   │       ├── find.rs
    │   │   │       ├── mod.rs
    │   │   │       └── world
    │   │   │           ├── action_steps.rs
    │   │   │           ├── check_steps.rs
    │   │   │           └── mod.rs
    │   └── src
    │   │   ├── app.rs
    │   │   ├── instrumented.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── tailwind_actix
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── end2end
    │   │   ├── package-lock.json
    │   │   ├── package.json
    │   │   ├── playwright.config.ts
    │   │   └── tests
    │   │   │   └── example.spec.ts
    │   ├── input.css
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── app.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── tailwind_axum
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── end2end
    │   │   ├── package-lock.json
    │   │   ├── package.json
    │   │   ├── playwright.config.ts
    │   │   └── tests
    │   │   │   └── example.spec.ts
    │   ├── input.css
    │   ├── package-lock.json
    │   ├── package.json
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── app.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── tailwind_csr
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── end2end
    │   │   ├── package-lock.json
    │   │   ├── package.json
    │   │   ├── playwright.config.ts
    │   │   └── tests
    │   │   │   └── example.spec.ts
    │   ├── index.html
    │   ├── input.css
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── app.rs
    │   │   └── main.rs
    ├── timer
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── todo_app_sqlite
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── Todos.db
    │   ├── e2e
    │   │   ├── Cargo.toml
    │   │   ├── Makefile.toml
    │   │   ├── README.md
    │   │   ├── features
    │   │   │   ├── add_todo.feature
    │   │   │   ├── delete_todo.feature
    │   │   │   └── open_app.feature
    │   │   └── tests
    │   │   │   ├── app_suite.rs
    │   │   │   └── fixtures
    │   │   │       ├── action.rs
    │   │   │       ├── check.rs
    │   │   │       ├── find.rs
    │   │   │       ├── mod.rs
    │   │   │       └── world
    │   │   │           ├── action_steps.rs
    │   │   │           ├── check_steps.rs
    │   │   │           └── mod.rs
    │   ├── migrations
    │   │   └── 20221118172000_create_todo_table.sql
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   └── todo.rs
    │   └── style.css
    ├── todo_app_sqlite_axum
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── Todos.db
    │   ├── e2e
    │   │   ├── Cargo.toml
    │   │   ├── Makefile.toml
    │   │   ├── README.md
    │   │   ├── features
    │   │   │   ├── add_todo.feature
    │   │   │   ├── delete_todo.feature
    │   │   │   └── open_app.feature
    │   │   └── tests
    │   │   │   ├── app_suite.rs
    │   │   │   └── fixtures
    │   │   │       ├── action.rs
    │   │   │       ├── check.rs
    │   │   │       ├── find.rs
    │   │   │       ├── mod.rs
    │   │   │       └── world
    │   │   │           ├── action_steps.rs
    │   │   │           ├── check_steps.rs
    │   │   │           └── mod.rs
    │   ├── migrations
    │   │   └── 20221118172000_create_todo_table.sql
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── error_template.rs
    │   │   ├── errors.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   └── todo.rs
    │   └── style.css
    ├── todo_app_sqlite_csr
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── Todos.db
    │   ├── e2e
    │   │   ├── Cargo.toml
    │   │   ├── Makefile.toml
    │   │   ├── README.md
    │   │   ├── features
    │   │   │   ├── add_todo.feature
    │   │   │   ├── delete_todo.feature
    │   │   │   └── open_app.feature
    │   │   └── tests
    │   │   │   ├── app_suite.rs
    │   │   │   └── fixtures
    │   │   │       ├── action.rs
    │   │   │       ├── check.rs
    │   │   │       ├── find.rs
    │   │   │       ├── mod.rs
    │   │   │       └── world
    │   │   │           ├── action_steps.rs
    │   │   │           ├── check_steps.rs
    │   │   │           └── mod.rs
    │   ├── migrations
    │   │   └── 20221118172000_create_todo_table.sql
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   ├── src
    │   │   ├── error_template.rs
    │   │   ├── errors.rs
    │   │   ├── fallback.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   └── todo.rs
    │   └── style.css
    ├── todomvc
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── node_modules
    │   │   ├── .package-lock.json
    │   │   ├── todomvc-app-css
    │   │   │   ├── index.css
    │   │   │   ├── license
    │   │   │   ├── package.json
    │   │   │   └── readme.md
    │   │   └── todomvc-common
    │   │   │   ├── base.css
    │   │   │   ├── base.js
    │   │   │   ├── license
    │   │   │   ├── package.json
    │   │   │   └── readme.md
    │   ├── package-lock.json
    │   ├── package.json
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    └── websocket
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── e2e
    │       ├── Cargo.toml
    │       ├── Makefile.toml
    │       ├── README.md
    │       ├── features
    │       │   ├── echo_client_error.feature
    │       │   ├── echo_server_error.feature
    │       │   ├── echo_text.feature
    │       │   └── open_app.feature
    │       └── tests
    │       │   ├── app_suite.rs
    │       │   └── fixtures
    │       │       ├── action.rs
    │       │       ├── check.rs
    │       │       ├── find.rs
    │       │       ├── mod.rs
    │       │       └── world
    │       │           ├── action_steps.rs
    │       │           ├── check_steps.rs
    │       │           └── mod.rs
    │   ├── public
    │       └── favicon.ico
    │   ├── src
    │       ├── lib.rs
    │       ├── main.rs
    │       └── websocket.rs
    │   └── style.css
├── flake.lock
├── flake.nix
├── hydration_context
    ├── Cargo.toml
    ├── Makefile.toml
    ├── README.md
    └── src
    │   ├── csr.rs
    │   ├── hydrate.rs
    │   ├── lib.rs
    │   └── ssr.rs
├── integrations
    ├── actix
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── src
    │   │   └── lib.rs
    │   └── tests
    │   │   └── extract_routes.rs
    ├── axum
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   └── src
    │   │   └── lib.rs
    └── utils
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   └── src
    │       └── lib.rs
├── leptos
    ├── Cargo.toml
    ├── Makefile.toml
    ├── build.rs
    ├── src
    │   ├── animated_show.rs
    │   ├── attribute_interceptor.rs
    │   ├── await_.rs
    │   ├── callback.rs
    │   ├── children.rs
    │   ├── component.rs
    │   ├── error_boundary.rs
    │   ├── for_loop.rs
    │   ├── form.rs
    │   ├── from_form_data.rs
    │   ├── hydration
    │   │   ├── hydration_script.js
    │   │   ├── island_script.js
    │   │   ├── islands_routing.js
    │   │   ├── mod.rs
    │   │   └── reload_script.js
    │   ├── into_view.rs
    │   ├── lib.rs
    │   ├── logging.rs
    │   ├── mount.rs
    │   ├── nonce.rs
    │   ├── portal.rs
    │   ├── provider.rs
    │   ├── show.rs
    │   ├── suspense_component.rs
    │   ├── text_prop.rs
    │   └── transition.rs
    └── tests
    │   ├── ssr.rs
    │   └── test_examples
    │       └── suspense-tests
    │           └── Cargo.lock
├── leptos_config
    ├── Cargo.toml
    ├── Makefile.toml
    ├── src
    │   ├── errors.rs
    │   ├── lib.rs
    │   └── tests.rs
    └── tests
    │   └── config.rs
├── leptos_dom
    ├── .cargo
    │   └── config.toml
    ├── Cargo.toml
    ├── Makefile.toml
    ├── examples
    │   ├── hydration-test
    │   │   ├── Cargo.toml
    │   │   └── src
    │   │   │   ├── lib.rs
    │   │   │   └── main.rs
    │   ├── test-bench
    │   │   ├── Cargo.toml
    │   │   ├── index.html
    │   │   └── src
    │   │   │   └── main.rs
    │   └── view-tests
    │   │   ├── Cargo.toml
    │   │   ├── index.html
    │   │   └── src
    │   │       └── main.rs
    └── src
    │   ├── helpers.rs
    │   ├── lib.rs
    │   ├── logging.rs
    │   └── macro_helpers
    │       ├── mod.rs
    │       └── tracing_property.rs
├── leptos_hot_reload
    ├── Cargo.toml
    ├── Makefile.toml
    └── src
    │   ├── diff.rs
    │   ├── lib.rs
    │   ├── node.rs
    │   ├── parsing.rs
    │   └── patch.js
├── leptos_macro
    ├── .gitignore
    ├── Cargo.toml
    ├── Makefile.toml
    ├── build.rs
    ├── example
    │   ├── Cargo.toml
    │   └── src
    │   │   └── lib.rs
    ├── src
    │   ├── component.rs
    │   ├── lazy.rs
    │   ├── lib.rs
    │   ├── memo.rs
    │   ├── params.rs
    │   ├── slice.rs
    │   ├── slot.rs
    │   └── view
    │   │   ├── component_builder.rs
    │   │   ├── mod.rs
    │   │   ├── slot_helper.rs
    │   │   ├── snapshots
    │   │       └── leptos_macro__view__tests__client_template__full_span__counter_component.snap
    │   │   └── utils.rs
    └── tests
    │   ├── component.rs
    │   ├── memo.rs
    │   ├── memo
    │       ├── red.rs
    │       └── red.stderr
    │   ├── params.rs
    │   ├── server.rs
    │   ├── slice.rs
    │   ├── slice
    │       ├── red.rs
    │       └── red.stderr
    │   ├── ui.rs
    │   └── ui
    │       ├── component.rs
    │       ├── component.stderr
    │       ├── component_absolute.rs
    │       ├── component_absolute.stderr
    │       ├── server.rs
    │       └── server.stderr
├── leptos_server
    ├── Cargo.toml
    ├── Makefile.toml
    └── src
    │   ├── action.rs
    │   ├── lib.rs
    │   ├── local_resource.rs
    │   ├── multi_action.rs
    │   ├── once_resource.rs
    │   ├── resource.rs
    │   ├── serializers.rs
    │   └── shared.rs
├── logos
    └── Simple_Icon.svg
├── meta
    ├── Cargo.toml
    ├── Makefile.toml
    └── src
    │   ├── body.rs
    │   ├── html.rs
    │   ├── lib.rs
    │   ├── link.rs
    │   ├── meta_tags.rs
    │   ├── script.rs
    │   ├── style.rs
    │   ├── stylesheet.rs
    │   └── title.rs
├── next_tuple
    ├── Cargo.toml
    ├── Makefile.toml
    ├── README.md
    └── src
    │   └── lib.rs
├── oco
    ├── Cargo.toml
    ├── Makefile.toml
    ├── README.md
    └── src
    │   └── lib.rs
├── or_poisoned
    ├── Cargo.toml
    ├── Makefile.toml
    ├── README.md
    └── src
    │   └── lib.rs
├── projects
    ├── README.md
    ├── bevy3d_ui
    │   ├── Cargo.toml
    │   ├── README.md
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   └── src
    │   │   ├── demos
    │   │       ├── bevydemo1
    │   │       │   ├── eventqueue
    │   │       │   │   ├── events.rs
    │   │       │   │   ├── mod.rs
    │   │       │   │   └── plugin.rs
    │   │       │   ├── mod.rs
    │   │       │   ├── scene.rs
    │   │       │   └── state.rs
    │   │       └── mod.rs
    │   │   ├── main.rs
    │   │   └── routes
    │   │       ├── demo1.rs
    │   │       └── mod.rs
    ├── counter_dwarf_debug
    │   ├── .gitignore
    │   ├── .vscode
    │   │   ├── launch.json
    │   │   └── tasks.json
    │   ├── Cargo.toml
    │   ├── README.md
    │   ├── Trunk.toml
    │   ├── img
    │   │   ├── breakpoint1.png
    │   │   └── breakpoint2.png
    │   ├── index.html
    │   ├── public
    │   │   └── favicon.ico
    │   ├── rust-toolchain.toml
    │   └── src
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── hexagonal-architecture
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── README.md
    │   ├── end2end
    │   │   ├── package-lock.json
    │   │   ├── package.json
    │   │   ├── playwright.config.ts
    │   │   ├── tests
    │   │   │   └── example.spec.ts
    │   │   └── tsconfig.json
    │   ├── leptos_hexagonal_architecture.png
    │   ├── public
    │   │   └── favicon.ico
    │   ├── src
    │   │   ├── app.rs
    │   │   ├── config.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   ├── middleware.rs
    │   │   ├── server_types.rs
    │   │   ├── trait_impl.rs
    │   │   ├── traits.rs
    │   │   └── ui_types.rs
    │   └── style
    │   │   └── main.scss
    ├── login_with_token_csr_only
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── api-boundary
    │   │   ├── Cargo.toml
    │   │   ├── Makefile.toml
    │   │   └── src
    │   │   │   └── lib.rs
    │   ├── client
    │   │   ├── Cargo.toml
    │   │   ├── Makefile.toml
    │   │   ├── Trunk.toml
    │   │   ├── index.html
    │   │   └── src
    │   │   │   ├── api.rs
    │   │   │   ├── components
    │   │   │       ├── credentials.rs
    │   │   │       ├── mod.rs
    │   │   │       └── navbar.rs
    │   │   │   ├── lib.rs
    │   │   │   ├── main.rs
    │   │   │   └── pages
    │   │   │       ├── home.rs
    │   │   │       ├── login.rs
    │   │   │       ├── mod.rs
    │   │   │       └── register.rs
    │   └── server
    │   │   ├── Cargo.toml
    │   │   ├── Makefile.toml
    │   │   └── src
    │   │       ├── adapters.rs
    │   │       ├── application.rs
    │   │       └── main.rs
    ├── meilisearch-searchbar
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── data_set.csv
    │   └── src
    │   │   ├── fallback.rs
    │   │   ├── lib.rs
    │   │   └── main.rs
    ├── nginx-mpmc
    │   ├── .gitignore
    │   ├── README.md
    │   ├── app-1
    │   │   ├── .gitignore
    │   │   ├── Cargo.toml
    │   │   ├── LICENSE
    │   │   ├── README.md
    │   │   ├── public
    │   │   │   └── favicon.ico
    │   │   ├── src
    │   │   │   ├── app.rs
    │   │   │   ├── error_template.rs
    │   │   │   ├── fileserv.rs
    │   │   │   ├── lib.rs
    │   │   │   └── main.rs
    │   │   └── style
    │   │   │   └── main.scss
    │   ├── app-2
    │   │   ├── .gitignore
    │   │   ├── Cargo.toml
    │   │   ├── LICENSE
    │   │   ├── README.md
    │   │   ├── public
    │   │   │   └── favicon.ico
    │   │   ├── src
    │   │   │   ├── app.rs
    │   │   │   ├── error_template.rs
    │   │   │   ├── fileserv.rs
    │   │   │   ├── lib.rs
    │   │   │   └── main.rs
    │   │   └── style
    │   │   │   └── main.scss
    │   ├── kill.sh
    │   ├── nginx.conf
    │   ├── nginx_linux.conf
    │   ├── run.sh
    │   ├── run_linux.sh
    │   ├── shared-server-1
    │   │   ├── .gitignore
    │   │   ├── Cargo.toml
    │   │   ├── LICENSE
    │   │   ├── README.md
    │   │   ├── public
    │   │   │   └── favicon.ico
    │   │   ├── src
    │   │   │   ├── lib.rs
    │   │   │   └── main.rs
    │   │   └── style
    │   │   │   └── main.scss
    │   └── shared-server-2
    │   │   ├── .gitignore
    │   │   ├── Cargo.toml
    │   │   ├── LICENSE
    │   │   ├── README.md
    │   │   ├── public
    │   │       └── favicon.ico
    │   │   ├── src
    │   │       ├── lib.rs
    │   │       └── main.rs
    │   │   └── style
    │   │       └── main.scss
    ├── openapi-openai-api-swagger-ui
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── README.md
    │   ├── end2end
    │   │   ├── package-lock.json
    │   │   ├── package.json
    │   │   ├── playwright.config.ts
    │   │   └── tests
    │   │   │   └── example.spec.ts
    │   ├── public
    │   │   └── favicon.ico
    │   ├── src
    │   │   ├── app.rs
    │   │   ├── error_template.rs
    │   │   ├── fileserv.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   └── open_ai.rs
    │   └── style
    │   │   └── main.scss
    ├── ory-kratos
    │   ├── .env
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── README.md
    │   ├── app
    │   │   ├── Cargo.toml
    │   │   └── src
    │   │   │   ├── auth
    │   │   │       ├── extractors.rs
    │   │   │       ├── kratos_error.rs
    │   │   │       ├── kratos_html.rs
    │   │   │       ├── login.rs
    │   │   │       ├── logout.rs
    │   │   │       ├── mod.rs
    │   │   │       ├── recovery.rs
    │   │   │       ├── registration.rs
    │   │   │       ├── session.rs
    │   │   │       ├── settings.rs
    │   │   │       └── verification.rs
    │   │   │   ├── database_calls.rs
    │   │   │   ├── error_template.rs
    │   │   │   ├── lib.rs
    │   │   │   └── posts
    │   │   │       ├── create_posts.rs
    │   │   │       ├── mod.rs
    │   │   │       ├── post.rs
    │   │   │       └── posts_page.rs
    │   ├── docker-compose.yml
    │   ├── e2e
    │   │   ├── Cargo.toml
    │   │   ├── features
    │   │   │   ├── 0_test.feature
    │   │   │   ├── 1_register.feature
    │   │   │   ├── 2_login.feature
    │   │   │   ├── 3_logout.feature
    │   │   │   ├── 4_recovery.feature
    │   │   │   ├── 5_settings.feature
    │   │   │   ├── 6_add_post.feature
    │   │   │   └── 7_edit_post.feature
    │   │   └── tests
    │   │   │   ├── app_suite.rs
    │   │   │   └── fixtures
    │   │   │       ├── mod.rs
    │   │   │       └── steps.rs
    │   ├── frontend
    │   │   ├── Cargo.toml
    │   │   └── src
    │   │   │   └── lib.rs
    │   ├── ids
    │   │   ├── Cargo.toml
    │   │   └── src
    │   │   │   └── lib.rs
    │   ├── kratos
    │   │   ├── email.schema.json
    │   │   └── kratos.yaml
    │   ├── migrations
    │   │   ├── 01_create_users.sql
    │   │   ├── 02_create_posts.sql
    │   │   └── 03_create_post_permissions.sql
    │   ├── public
    │   │   ├── apple_sso_btn.png
    │   │   └── favicon.ico
    │   ├── server
    │   │   ├── Cargo.toml
    │   │   └── src
    │   │   │   ├── extract_session.rs
    │   │   │   ├── fileserv.rs
    │   │   │   └── main.rs
    │   └── style
    │   │   └── main.scss
    ├── session_auth_axum
    │   ├── .env
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── Todos.db
    │   ├── flake.lock
    │   ├── flake.nix
    │   ├── migrations
    │   │   └── 20230226000000_create_todo_table.sql
    │   ├── public
    │   │   └── favicon.ico
    │   ├── src
    │   │   ├── auth.rs
    │   │   ├── error_template.rs
    │   │   ├── errors.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   ├── state.rs
    │   │   └── todo.rs
    │   └── style.css
    ├── sitemap_axum
    │   ├── .env.example
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── init
    │   │   └── schema.sql
    │   ├── public
    │   │   └── favicon.ico
    │   ├── sitemap-index.xml
    │   ├── sitemap-static.xml
    │   └── src
    │   │   ├── app.rs
    │   │   ├── error_template.rs
    │   │   ├── fileserv.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   └── sitemap.rs
    ├── sso_auth_axum
    │   ├── Cargo.toml
    │   ├── LICENSE
    │   ├── Makefile.toml
    │   ├── README.md
    │   ├── flake.lock
    │   ├── flake.nix
    │   ├── migrations
    │   │   └── 20231217000000_create_tables.sql
    │   ├── public
    │   │   └── favicon.ico
    │   ├── src
    │   │   ├── auth.rs
    │   │   ├── error_template.rs
    │   │   ├── fallback.rs
    │   │   ├── lib.rs
    │   │   ├── main.rs
    │   │   ├── sign_in_sign_up.rs
    │   │   └── state.rs
    │   ├── sso.db
    │   └── style.css
    └── tauri-from-scratch
    │   ├── .gitignore
    │   ├── Cargo.toml
    │   ├── README.md
    │   ├── Trunk.toml
    │   ├── public
    │       └── favicon.ico
    │   ├── src-orig
    │       ├── Cargo.toml
    │       ├── index.html
    │       └── src
    │       │   ├── fallback.rs
    │       │   ├── lib.rs
    │       │   └── main.rs
    │   └── src-tauri
    │       ├── Cargo.toml
    │       ├── build.rs
    │       ├── icons
    │           └── icon.png
    │       ├── src
    │           ├── lib.rs
    │           └── main.rs
    │       └── tauri.conf.json
├── reactive_graph
    ├── Cargo.toml
    ├── Makefile.toml
    ├── README.md
    ├── build.rs
    ├── src
    │   ├── actions
    │   │   ├── action.rs
    │   │   ├── mod.rs
    │   │   └── multi_action.rs
    │   ├── channel.rs
    │   ├── computed.rs
    │   ├── computed
    │   │   ├── arc_memo.rs
    │   │   ├── async_derived
    │   │   │   ├── arc_async_derived.rs
    │   │   │   ├── async_derived.rs
    │   │   │   ├── future_impls.rs
    │   │   │   ├── inner.rs
    │   │   │   └── mod.rs
    │   │   ├── inner.rs
    │   │   ├── memo.rs
    │   │   └── selector.rs
    │   ├── diagnostics.rs
    │   ├── effect.rs
    │   ├── effect
    │   │   ├── effect.rs
    │   │   ├── effect_function.rs
    │   │   ├── immediate.rs
    │   │   ├── inner.rs
    │   │   └── render_effect.rs
    │   ├── graph.rs
    │   ├── graph
    │   │   ├── node.rs
    │   │   ├── sets.rs
    │   │   ├── source.rs
    │   │   └── subscriber.rs
    │   ├── lib.rs
    │   ├── nightly.rs
    │   ├── owner.rs
    │   ├── owner
    │   │   ├── arc_stored_value.rs
    │   │   ├── arena.rs
    │   │   ├── arena_item.rs
    │   │   ├── context.rs
    │   │   ├── storage.rs
    │   │   └── stored_value.rs
    │   ├── send_wrapper_ext.rs
    │   ├── serde.rs
    │   ├── signal.rs
    │   ├── signal
    │   │   ├── arc_read.rs
    │   │   ├── arc_rw.rs
    │   │   ├── arc_trigger.rs
    │   │   ├── arc_write.rs
    │   │   ├── guards.rs
    │   │   ├── mapped.rs
    │   │   ├── read.rs
    │   │   ├── rw.rs
    │   │   ├── subscriber_traits.rs
    │   │   ├── trigger.rs
    │   │   └── write.rs
    │   ├── trait_options.rs
    │   ├── traits.rs
    │   ├── transition.rs
    │   └── wrappers.rs
    └── tests
    │   ├── async_derived.rs
    │   ├── cleanup.rs
    │   ├── effect.rs
    │   ├── effect_immediate.rs
    │   ├── memo.rs
    │   ├── signal.rs
    │   └── watch.rs
├── reactive_stores
    ├── Cargo.toml
    ├── Makefile.toml
    ├── README.md
    └── src
    │   ├── arc_field.rs
    │   ├── deref.rs
    │   ├── field.rs
    │   ├── iter.rs
    │   ├── keyed.rs
    │   ├── len.rs
    │   ├── lib.rs
    │   ├── option.rs
    │   ├── patch.rs
    │   ├── path.rs
    │   ├── store_field.rs
    │   └── subfield.rs
├── reactive_stores_macro
    ├── Cargo.toml
    ├── Makefile.toml
    ├── README.md
    └── src
    │   └── lib.rs
├── router
    ├── Cargo.toml
    ├── Makefile.toml
    ├── build.rs
    └── src
    │   ├── components.rs
    │   ├── flat_router.rs
    │   ├── form.rs
    │   ├── generate_route_list.rs
    │   ├── hooks.rs
    │   ├── lib.rs
    │   ├── link.rs
    │   ├── location
    │       ├── history.rs
    │       ├── mod.rs
    │       └── server.rs
    │   ├── matching
    │       ├── any_choose_view.rs
    │       ├── choose_view.rs
    │       ├── horizontal
    │       │   ├── mod.rs
    │       │   ├── param_segments.rs
    │       │   ├── static_segment.rs
    │       │   └── tuples.rs
    │       ├── mod.rs
    │       ├── nested
    │       │   ├── any_nested_match.rs
    │       │   ├── any_nested_route.rs
    │       │   ├── mod.rs
    │       │   └── tuples.rs
    │       ├── path_segment.rs
    │       ├── resolve_path.rs
    │       └── vertical
    │       │   └── mod.rs
    │   ├── method.rs
    │   ├── navigate.rs
    │   ├── nested_router.rs
    │   ├── params.rs
    │   ├── reactive.rs
    │   ├── ssr_mode.rs
    │   └── static_routes.rs
├── router_macro
    ├── Cargo.toml
    ├── Makefile.toml
    ├── src
    │   └── lib.rs
    └── tests
    │   └── path.rs
├── rustfmt.toml
├── scripts
    └── update_nightly.sh
├── server_fn
    ├── Cargo.lock
    ├── Cargo.toml
    ├── Makefile.toml
    ├── build.rs
    ├── server_fn_macro_default
    │   ├── Cargo.toml
    │   ├── Makefile.toml
    │   └── src
    │   │   └── lib.rs
    ├── src
    │   ├── client.rs
    │   ├── codec
    │   │   ├── cbor.rs
    │   │   ├── json.rs
    │   │   ├── mod.rs
    │   │   ├── msgpack.rs
    │   │   ├── multipart.rs
    │   │   ├── patch.rs
    │   │   ├── post.rs
    │   │   ├── postcard.rs
    │   │   ├── put.rs
    │   │   ├── rkyv.rs
    │   │   ├── serde_lite.rs
    │   │   ├── stream.rs
    │   │   └── url.rs
    │   ├── error.rs
    │   ├── lib.rs
    │   ├── middleware
    │   │   └── mod.rs
    │   ├── redirect.rs
    │   ├── request
    │   │   ├── actix.rs
    │   │   ├── axum.rs
    │   │   ├── browser.rs
    │   │   ├── generic.rs
    │   │   ├── mod.rs
    │   │   ├── reqwest.rs
    │   │   └── spin.rs
    │   ├── response
    │   │   ├── actix.rs
    │   │   ├── browser.rs
    │   │   ├── generic.rs
    │   │   ├── http.rs
    │   │   ├── mod.rs
    │   │   └── reqwest.rs
    │   └── server.rs
    └── tests
    │   ├── invalid
    │       ├── aliased_return_full.rs
    │       ├── aliased_return_full.stderr
    │       ├── aliased_return_none.rs
    │       ├── aliased_return_none.stderr
    │       ├── aliased_return_part.rs
    │       ├── aliased_return_part.stderr
    │       ├── empty_return.rs
    │       ├── empty_return.stderr
    │       ├── no_return.rs
    │       ├── no_return.stderr
    │       ├── not_async.rs
    │       ├── not_async.stderr
    │       ├── not_result.rs
    │       └── not_result.stderr
    │   ├── server_macro.rs
    │   └── valid
    │       ├── aliased_return_full.rs
    │       ├── aliased_return_none.rs
    │       ├── aliased_return_part.rs
    │       ├── custom_error_aliased_return_full.rs
    │       ├── custom_error_aliased_return_none.rs
    │       └── custom_error_aliased_return_part.rs
├── server_fn_macro
    ├── Cargo.toml
    ├── Makefile.toml
    ├── build.rs
    └── src
    │   └── lib.rs
└── tachys
    ├── Cargo.toml
    ├── Makefile.toml
    ├── build.rs
    └── src
        ├── dom.rs
        ├── erased.rs
        ├── html
            ├── attribute
            │   ├── any_attribute.rs
            │   ├── aria.rs
            │   ├── custom.rs
            │   ├── global.rs
            │   ├── key.rs
            │   ├── maybe_next_attr_erasure_macros.rs
            │   ├── mod.rs
            │   └── value.rs
            ├── class.rs
            ├── directive.rs
            ├── element
            │   ├── custom.rs
            │   ├── element_ext.rs
            │   ├── elements.rs
            │   ├── inner_html.rs
            │   └── mod.rs
            ├── event.rs
            ├── islands.rs
            ├── mod.rs
            ├── node_ref.rs
            ├── property.rs
            └── style.rs
        ├── hydration.rs
        ├── lib.rs
        ├── mathml
            └── mod.rs
        ├── oco.rs
        ├── reactive_graph
            ├── bind.rs
            ├── class.rs
            ├── inner_html.rs
            ├── mod.rs
            ├── node_ref.rs
            ├── owned.rs
            ├── property.rs
            ├── style.rs
            └── suspense.rs
        ├── renderer
            ├── dom.rs
            ├── mock_dom.rs
            ├── mod.rs
            └── sledgehammer.rs
        ├── ssr
            └── mod.rs
        ├── svg
            └── mod.rs
        └── view
            ├── add_attr.rs
            ├── any_view.rs
            ├── either.rs
            ├── error_boundary.rs
            ├── fragment.rs
            ├── iterators.rs
            ├── keyed.rs
            ├── mod.rs
            ├── primitives.rs
            ├── static_types.rs
            ├── strings.rs
            ├── template.rs
            └── tuples.rs


/.github/FUNDING.yml:
--------------------------------------------------------------------------------
1 | # These are supported funding model platforms
2 | 
3 | github: gbj
4 | 


--------------------------------------------------------------------------------
/.github/ISSUE_TEMPLATE/config.yml:
--------------------------------------------------------------------------------
1 | contact_links:
2 |   - name: Support or Question
3 |     url: https://github.com/leptos-rs/leptos/discussions/new?category=q-a
4 |     about: Do you need help figuring out how to do something, or want some help troubleshooting a bug? You can ask in our Discussions section.
5 |   - name: Discord Discussions
6 |     url: https://discord.gg/YdRAhS7eQB
7 |     about: For more informal, real-time conversation and support, you can join our Discord server.
8 | 


--------------------------------------------------------------------------------
/.gitignore:
--------------------------------------------------------------------------------
 1 | target
 2 | dist
 3 | pkg
 4 | comparisons
 5 | blob.rs
 6 | **/projects/**/Cargo.lock
 7 | **/examples/**/Cargo.lock
 8 | **/benchmarks/**/Cargo.lock
 9 | **/*.rs.bk
10 | .DS_Store
11 | .idea
12 | .direnv
13 | .envrc
14 | 
15 | .vscode
16 | vendor
17 | hash.txt
18 | 


--------------------------------------------------------------------------------
/any_error/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "throw_error"
 3 | version = "0.3.0"
 4 | authors = ["Greg Johnston"]
 5 | license = "MIT"
 6 | readme = "../README.md"
 7 | repository = "https://github.com/leptos-rs/leptos"
 8 | description = "Utilities for wrapping, throwing, and catching errors."
 9 | rust-version.workspace = true
10 | edition.workspace = true
11 | 
12 | [dependencies]
13 | pin-project-lite = { workspace = true, default-features = true }
14 | 


--------------------------------------------------------------------------------
/any_error/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/any_error/README.md:
--------------------------------------------------------------------------------
1 | A utility library for wrapping arbitrary errors, and for “throwing” errors in a way
2 | that can be caught by user-defined error hooks.
3 | 


--------------------------------------------------------------------------------
/any_spawner/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |   { path = "../cargo-make/main.toml" },
3 |   { path = "../cargo-make/wasm-test.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/benchmarks/src/lib.rs:
--------------------------------------------------------------------------------
1 | #![feature(test)]
2 | 
3 | extern crate test;
4 | 
5 | mod reactive;
6 | mod ssr;
7 | mod todomvc;
8 | 


--------------------------------------------------------------------------------
/cargo-make/main.toml:
--------------------------------------------------------------------------------
 1 | extend = [{ path = "./lint.toml" }, { path = "./test.toml" }]
 2 | 
 3 | [env]
 4 | RUSTFLAGS = ""
 5 | LEPTOS_OUTPUT_NAME = "ci" # allows examples to check/build without cargo-leptos
 6 | 
 7 | [env.github-actions]
 8 | RUSTFLAGS = "-D warnings"
 9 | 
10 | [tasks.ci]
11 | dependencies = ["lint", "test-each-feature", "doctests"]
12 | 


--------------------------------------------------------------------------------
/cargo-make/wasm-test.toml:
--------------------------------------------------------------------------------
1 | [tasks.post-test]
2 | dependencies = ["test-wasm"]
3 | 
4 | [tasks.test-wasm]
5 | env = { CARGO_MAKE_WASM_TEST_ARGS = "--headless --chrome --features=wasm-bindgen" }
6 | command = "cargo"
7 | args = ["make", "wasm-pack-test"]
8 | 


--------------------------------------------------------------------------------
/const_str_slice_concat/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "const_str_slice_concat"
 3 | version = "0.1.0"
 4 | authors = ["Greg Johnston"]
 5 | license = "MIT"
 6 | readme = "../README.md"
 7 | repository = "https://github.com/leptos-rs/leptos"
 8 | description = "Utilities for const concatenation of string slices."
 9 | rust-version.workspace = true
10 | edition.workspace = true
11 | 
12 | [dependencies]
13 | 


--------------------------------------------------------------------------------
/const_str_slice_concat/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/docs/book/.gitignore:
--------------------------------------------------------------------------------
1 | book


--------------------------------------------------------------------------------
/docs/book/README.md:
--------------------------------------------------------------------------------
1 | The Leptos book is now available at [https://book.leptos.dev](https://book.leptos.dev).
2 | 
3 | The source code for the book has moved to [https://github.com/leptos-rs/book](https://github.com/leptos-rs/book). Please open issues or make PRs in that repository.
4 | 


--------------------------------------------------------------------------------
/docs/book/book.toml:
--------------------------------------------------------------------------------
 1 | [output.html]
 2 | additional-css = ["./mdbook-admonish.css"]
 3 | [output.html.playground]
 4 | runnable = false
 5 | 
 6 | [preprocessor]
 7 | 
 8 | [preprocessor.admonish]
 9 | command = "mdbook-admonish"
10 | assets_version = "3.0.1" # do not edit: managed by `mdbook-admonish install`
11 | 


--------------------------------------------------------------------------------
/docs/book/src/01_introduction.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/">
2 | <link rel="canonical" href="https://book.leptos.dev/">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/15_global_state.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/15_global_state.html">
2 | <link rel="canonical" href="https://book.leptos.dev/15_global_state.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/appendix_reactive_graph.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/appendix_reactive_graph.html">
2 | <link rel="canonical" href="https://book.leptos.dev/appendix_reactive_graph.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/async/10_resources.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/async/10_resources.html">
2 | <link rel="canonical" href="https://book.leptos.dev/async/10_resources.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/async/11_suspense.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/async/11_suspense.html">
2 | <link rel="canonical" href="https://book.leptos.dev/async/11_suspense.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/async/12_transition.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/async/12_transition.html">
2 | <link rel="canonical" href="https://book.leptos.dev/async/12_transition.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/async/13_actions.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/async/13_action.html">
2 | <link rel="canonical" href="https://book.leptos.dev/async/13_action.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/async/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/async/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/async/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/csr_wrapping_up.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/csr_wrapping_up.html">
2 | <link rel="canonical" href="https://book.leptos.dev/csr_wrapping_up.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/deployment/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/deployment/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/deployment/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/deployment/binary_size.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/deployment/binary_size.html">
2 | <link rel="canonical" href="https://book.leptos.dev/deployment/binary_size.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/getting_started/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/getting_started/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/getting_started/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/getting_started/community_crates.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/getting_started/community_crates.html">
2 | <link rel="canonical" href="https://book.leptos.dev/getting_started/community_crates.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/getting_started/leptos_dx.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/getting_started/leptos_dx.html">
2 | <link rel="canonical" href="https://book.leptos.dev/getting_started/leptos_dx.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/interlude_projecting_children.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/interlude_projecting_children.html">
2 | <link rel="canonical" href="https://book.leptos.dev/interlude_projecting_children.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/interlude_styling.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/interlude_styling.html">
2 | <link rel="canonical" href="https://book.leptos.dev/interlude_styling.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/islands.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/islands.html">
2 | <link rel="canonical" href="https://book.leptos.dev/islands.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/metadata.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/metadata.html">
2 | <link rel="canonical" href="https://book.leptos.dev/metadata.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/progressive_enhancement/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/progressive_enhancement/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/progressive_enhancement/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/progressive_enhancement/action_form.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/progressive_enhancement/action_form.html">
2 | <link rel="canonical" href="https://book.leptos.dev/progressive_enhancement/action_form.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/reactivity/14_create_effect.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/reactivity/14_create_effect.html">
2 | <link rel="canonical" href="https://book.leptos.dev/reactivity/14_create_effect.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/reactivity/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/reactivity/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/reactivity/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/reactivity/interlude_functions.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/reactivity/interlude_functions.html">
2 | <link rel="canonical" href="https://book.leptos.dev/reactivity/interlude_functions.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/reactivity/working_with_signals.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/reactivity/working_with_signals.html">
2 | <link rel="canonical" href="https://book.leptos.dev/reactivity/working_with_signals.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/router/16_routes.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/16_routes.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/16_routes.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/router/17_nested_routing.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/17_nested_routing.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/17_nested_routing.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/router/18_params_and_queries.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/18_params_and_queries.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/18_params_and_queries.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/router/19_a.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/19_a.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/19_a.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/router/20_form.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/20_form.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/20_form.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/router/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/server/25_server_functions.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/server/25_server_functions.html">
2 | <link rel="canonical" href="https://book.leptos.dev/server/25_server_functions.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/server/26_extractors.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/server/26_extractors.html">
2 | <link rel="canonical" href="https://book.leptos.dev/server/26_extractors.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/server/27_response.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/server/27_response.html">
2 | <link rel="canonical" href="https://book.leptos.dev/server/27_response.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/server/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/server/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/server/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/ssr/21_cargo_leptos.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/ssr/21_cargo_leptos.html">
2 | <link rel="canonical" href="https://book.leptos.dev/ssr/21_cargo_leptos.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/ssr/22_life_cycle.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/ssr/22_life_cycle.html">
2 | <link rel="canonical" href="https://book.leptos.dev/ssr/22_life_cycle.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/ssr/23_ssr_modes.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/ssr/23_ssr_modes.html">
2 | <link rel="canonical" href="https://book.leptos.dev/ssr/23_ssr_modes.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/ssr/24_hydration_bugs.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/ssr/24_hydration_bugs.html">
2 | <link rel="canonical" href="https://book.leptos.dev/ssr/24_hydration_bugs.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/ssr/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/ssr/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/ssr/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/testing.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/testing.html">
2 | <link rel="canonical" href="https://book.leptos.dev/testing.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/01_basic_component.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/01_basic_component.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/01_basic_component.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/02_dynamic_attributes.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/02_dynamic_attributes.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/02_dynamic_attributes.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/03_components.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/03_components.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/03_components.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/04_iteration.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/04_iteration.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/04_iteration.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/04b_iteration.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/04b_iteration.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/04b_iteration.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/05_forms.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/05_forms.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/05_forms.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/06_control_flow.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/06_control_flow.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/06_control_flow.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/07_errors.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/07_errors.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/07_errors.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/08_parent_child.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/08_parent_child.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/08_parent_child.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/09_component_children.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/09_component_children.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/09_component_children.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book/src/view/builder.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/builder.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/builder.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/.gitignore:
--------------------------------------------------------------------------------
1 | book


--------------------------------------------------------------------------------
/docs/book_ru/README.md:
--------------------------------------------------------------------------------
1 | Перевод в процессе, книга скоро будет доступна
2 | > Translation underway, book will be available soon


--------------------------------------------------------------------------------
/docs/book_ru/book.toml:
--------------------------------------------------------------------------------
 1 | [output.html]
 2 | additional-css = ["./mdbook-admonish.css"]
 3 | [output.html.playground]
 4 | runnable = false
 5 | 
 6 | [preprocessor]
 7 | 
 8 | [preprocessor.admonish]
 9 | command = "mdbook-admonish"
10 | assets_version = "3.0.1" # не редактировать: управляется `mdbook-admonish install`
11 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/01_introduction.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/">
2 | <link rel="canonical" href="https://book.leptos.dev/">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/15_global_state.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/15_global_state.html">
2 | <link rel="canonical" href="https://book.leptos.dev/15_global_state.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/appendix_reactive_graph.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/appendix_reactive_graph.html">
2 | <link rel="canonical" href="https://book.leptos.dev/appendix_reactive_graph.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/async/10_resources.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/async/10_resources.html">
2 | <link rel="canonical" href="https://book.leptos.dev/async/10_resources.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/async/11_suspense.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/async/11_suspense.html">
2 | <link rel="canonical" href="https://book.leptos.dev/async/11_suspense.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/async/12_transition.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/async/12_transition.html">
2 | <link rel="canonical" href="https://book.leptos.dev/async/12_transition.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/async/13_actions.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/async/13_action.html">
2 | <link rel="canonical" href="https://book.leptos.dev/async/13_action.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/async/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/async/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/async/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/csr_wrapping_up.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/csr_wrapping_up.html">
2 | <link rel="canonical" href="https://book.leptos.dev/csr_wrapping_up.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/deployment/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/deployment/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/deployment/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/deployment/binary_size.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/deployment/binary_size.html">
2 | <link rel="canonical" href="https://book.leptos.dev/deployment/binary_size.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/getting_started/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/getting_started/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/getting_started/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/getting_started/community_crates.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/getting_started/community_crates.html">
2 | <link rel="canonical" href="https://book.leptos.dev/getting_started/community_crates.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/getting_started/leptos_dx.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/getting_started/leptos_dx.html">
2 | <link rel="canonical" href="https://book.leptos.dev/getting_started/leptos_dx.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/interlude_projecting_children.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/interlude_projecting_children.html">
2 | <link rel="canonical" href="https://book.leptos.dev/interlude_projecting_children.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/interlude_styling.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/interlude_styling.html">
2 | <link rel="canonical" href="https://book.leptos.dev/interlude_styling.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/islands.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/islands.html">
2 | <link rel="canonical" href="https://book.leptos.dev/islands.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/metadata.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/metadata.html">
2 | <link rel="canonical" href="https://book.leptos.dev/metadata.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/progressive_enhancement/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/progressive_enhancement/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/progressive_enhancement/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/progressive_enhancement/action_form.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/progressive_enhancement/action_form.html">
2 | <link rel="canonical" href="https://book.leptos.dev/progressive_enhancement/action_form.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/reactivity/14_create_effect.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/reactivity/14_create_effect.html">
2 | <link rel="canonical" href="https://book.leptos.dev/reactivity/14_create_effect.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/reactivity/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/reactivity/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/reactivity/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/reactivity/interlude_functions.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/reactivity/interlude_functions.html">
2 | <link rel="canonical" href="https://book.leptos.dev/reactivity/interlude_functions.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/reactivity/working_with_signals.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/reactivity/working_with_signals.html">
2 | <link rel="canonical" href="https://book.leptos.dev/reactivity/working_with_signals.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/router/16_routes.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/16_routes.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/16_routes.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/router/17_nested_routing.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/17_nested_routing.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/17_nested_routing.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/router/18_params_and_queries.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/18_params_and_queries.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/18_params_and_queries.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/router/19_a.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/19_a.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/19_a.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/router/20_form.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/20_form.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/20_form.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/router/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/router/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/router/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/server/25_server_functions.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/server/25_server_functions.html">
2 | <link rel="canonical" href="https://book.leptos.dev/server/25_server_functions.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/server/26_extractors.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/server/26_extractors.html">
2 | <link rel="canonical" href="https://book.leptos.dev/server/26_extractors.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/server/27_response.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/server/27_response.html">
2 | <link rel="canonical" href="https://book.leptos.dev/server/27_response.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/server/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/server/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/server/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/ssr/21_cargo_leptos.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/ssr/21_cargo_leptos.html">
2 | <link rel="canonical" href="https://book.leptos.dev/ssr/21_cargo_leptos.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/ssr/22_life_cycle.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/ssr/22_life_cycle.html">
2 | <link rel="canonical" href="https://book.leptos.dev/ssr/22_life_cycle.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/ssr/23_ssr_modes.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/ssr/23_ssr_modes.html">
2 | <link rel="canonical" href="https://book.leptos.dev/ssr/23_ssr_modes.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/ssr/24_hydration_bugs.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/ssr/24_hydration_bugs.html">
2 | <link rel="canonical" href="https://book.leptos.dev/ssr/24_hydration_bugs.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/ssr/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/ssr/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/ssr/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/testing.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/testing.html">
2 | <link rel="canonical" href="https://book.leptos.dev/testing.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/01_basic_component.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/01_basic_component.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/01_basic_component.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/02_dynamic_attributes.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/02_dynamic_attributes.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/02_dynamic_attributes.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/03_components.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/03_components.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/03_components.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/04_iteration.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/04_iteration.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/04_iteration.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/04b_iteration.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/04b_iteration.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/04b_iteration.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/05_forms.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/05_forms.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/05_forms.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/06_control_flow.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/06_control_flow.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/06_control_flow.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/07_errors.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/07_errors.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/07_errors.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/08_parent_child.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/08_parent_child.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/08_parent_child.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/09_component_children.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/09_component_children.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/09_component_children.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/README.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/index.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/index.html">
3 | 


--------------------------------------------------------------------------------
/docs/book_ru/src/view/builder.md:
--------------------------------------------------------------------------------
1 | <meta http-equiv="refresh" content="0; URL=https://book.leptos.dev/view/builder.html">
2 | <link rel="canonical" href="https://book.leptos.dev/view/builder.html">
3 | 


--------------------------------------------------------------------------------
/docs/logos/.gitignore:
--------------------------------------------------------------------------------
1 | logo-working.svg


--------------------------------------------------------------------------------
/docs/logos/Leptos_logo_RGB.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/docs/logos/Leptos_logo_RGB.png


--------------------------------------------------------------------------------
/docs/logos/Leptos_logo_abbreviation__circle_RGB.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/docs/logos/Leptos_logo_abbreviation__circle_RGB.png


--------------------------------------------------------------------------------
/docs/logos/Leptos_logo_abbreviation__square_RGB.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/docs/logos/Leptos_logo_abbreviation__square_RGB.png


--------------------------------------------------------------------------------
/docs/video/async.mov:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/docs/video/async.mov


--------------------------------------------------------------------------------
/docs/video/in-order.mov:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/docs/video/in-order.mov


--------------------------------------------------------------------------------
/docs/video/out-of-order.mov:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/docs/video/out-of-order.mov


--------------------------------------------------------------------------------
/either_of/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/either_of/README.md:
--------------------------------------------------------------------------------
1 | Utilities for working with enumerated types that contain one of `2..n` other types.
2 | 


--------------------------------------------------------------------------------
/examples/action-form-error-handling/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "action_form_error_handling"
9 | 


--------------------------------------------------------------------------------
/examples/action-form-error-handling/README.md:
--------------------------------------------------------------------------------
 1 | # Action Form Error Handling Example
 2 | 
 3 | ## Getting Started
 4 | 
 5 | See the [Examples README](../README.md) for setup and run instructions.
 6 | 
 7 | ## Quick Start
 8 | 
 9 | Execute `cargo leptos watch` to run this example.
10 | 


--------------------------------------------------------------------------------
/examples/action-form-error-handling/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod app;
 2 | 
 3 | #[cfg(feature = "hydrate")]
 4 | #[wasm_bindgen::prelude::wasm_bindgen]
 5 | pub fn hydrate() {
 6 |     use app::*;
 7 | 
 8 |     console_error_panic_hook::set_once();
 9 | 
10 |     leptos::mount::hydrate_body(App);
11 | }
12 | 


--------------------------------------------------------------------------------
/examples/action-form-error-handling/style/main.scss:
--------------------------------------------------------------------------------
 1 | body {
 2 | 	font-family: sans-serif;
 3 | 	text-align: center;
 4 | }
 5 | 
 6 | #app {
 7 | 	text-align: center;
 8 | }
 9 | 
10 | .form {
11 | 	display: flex;
12 | 	flex-direction: column;
13 | 	align-items: center;
14 | 	gap: 0.5rem;
15 | }
16 | 


--------------------------------------------------------------------------------
/examples/axum_js_ssr/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "axum_js_ssr"
9 | 


--------------------------------------------------------------------------------
/examples/axum_js_ssr/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Axum JS SSR Example
 2 | 
 3 | This example shows the various ways that JavaScript may be included into
 4 | a Leptos application.  The intent is to demonstrate how this may be done
 5 | and how it may cause the application to fail in an unexpected manner if
 6 | done incorrectly.
 7 | 
 8 | ## Quick Start
 9 | 
10 | Run `cargo leptos watch` to run this example.
11 | 


--------------------------------------------------------------------------------
/examples/axum_js_ssr/assets/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/axum_js_ssr/assets/favicon.ico


--------------------------------------------------------------------------------
/examples/axum_js_ssr/package.json:
--------------------------------------------------------------------------------
1 | {
2 |   "name": "axum_js_ssr",
3 |   "dependencies": {
4 |     "@highlightjs/cdn-assets": "^11.10.0"
5 |   }
6 | }
7 | 


--------------------------------------------------------------------------------
/examples/axum_js_ssr/src/api.rs:
--------------------------------------------------------------------------------
1 | use leptos::{prelude::ServerFnError, server};
2 | 
3 | #[server]
4 | pub async fn fetch_code() -> Result<String, ServerFnError> {
5 |     // emulate loading of code from a database/version control/etc
6 |     tokio::time::sleep(std::time::Duration::from_millis(50)).await;
7 |     Ok(crate::consts::CH05_02A.to_string())
8 | }
9 | 


--------------------------------------------------------------------------------
/examples/cargo-make/cargo-leptos-test.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "./cargo-leptos.toml" }
2 | 
3 | [tasks.integration-test]
4 | dependencies = ["install-cargo-leptos", "cargo-leptos-e2e"]
5 | 


--------------------------------------------------------------------------------
/examples/cargo-make/cargo-leptos-webdriver-test.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "./cargo-leptos.toml" },
3 |     { path = "../cargo-make/webdriver.toml" },
4 | ]
5 | 
6 | [tasks.integration-test]
7 | dependencies = ["install-cargo-leptos", "start-webdriver", "cargo-leptos-e2e"]
8 | 


--------------------------------------------------------------------------------
/examples/cargo-make/compile.toml:
--------------------------------------------------------------------------------
 1 | [tasks.cargo-all-features]
 2 | install_script = '''
 3 | cargo install --git https://github.com/sabify/cargo-all-features --branch arbitrary-command-support
 4 | '''
 5 | 
 6 | [tasks.build]
 7 | dependencies = ["cargo-all-features"]
 8 | command = "cargo"
 9 | args = ["all-features", "build"]
10 | 


--------------------------------------------------------------------------------
/examples/cargo-make/playwright-test.toml:
--------------------------------------------------------------------------------
1 | extend = [{ path = "../cargo-make/playwright.toml" }]
2 | 
3 | [tasks.integration-test]
4 | dependencies = ["test-playwright-autostart"]
5 | 


--------------------------------------------------------------------------------
/examples/cargo-make/playwright-trunk-test.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |   { path = "../cargo-make/playwright.toml" },
3 |   { path = "../cargo-make/trunk_server.toml" },
4 | ]
5 | 
6 | [tasks.integration-test]
7 | dependencies = ["build", "start-client", "test-playwright"]
8 | description = "Run integration test with automated start and stop of processes"
9 | 


--------------------------------------------------------------------------------
/examples/cargo-make/process.toml:
--------------------------------------------------------------------------------
 1 | extend = [
 2 |     { path = "./client-process.toml" },
 3 |     { path = "./server-process.toml" },
 4 | ]
 5 | 
 6 | [tasks.start]
 7 | dependencies = ["maybe-start-server", "maybe-start-client"]
 8 | 
 9 | [tasks.status]
10 | dependencies = ["server-status", "client-status"]
11 | 
12 | [tasks.stop]
13 | dependencies = ["stop-client", "stop-server"]
14 | 


--------------------------------------------------------------------------------
/examples/cargo-make/trunk_server.toml:
--------------------------------------------------------------------------------
 1 | [env]
 2 | CLIENT_PROCESS_NAME = "trunk"
 3 | 
 4 | [tasks.build]
 5 | command = "trunk"
 6 | args = ["build"]
 7 | 
 8 | [tasks.start-client]
 9 | script = '''
10 | trunk serve -q "${@}" &
11 | '''
12 | 


--------------------------------------------------------------------------------
/examples/cargo-make/wasm-test.toml:
--------------------------------------------------------------------------------
1 | [tasks.post-test]
2 | dependencies = ["test-wasm"]
3 | 
4 | [tasks.test-wasm]
5 | env = { CARGO_MAKE_WASM_TEST_ARGS = "--headless --chrome" }
6 | command = "cargo"
7 | args = ["make", "wasm-pack-test"]
8 | 


--------------------------------------------------------------------------------
/examples/counter/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "counter"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [profile.release]
 7 | opt-level = 'z'
 8 | codegen-units = 1
 9 | lto = true
10 | 
11 | [dependencies]
12 | leptos = { path = "../../leptos", features = ["csr"] }
13 | console_log = "1.0"
14 | log = "0.4.22"
15 | console_error_panic_hook = "0.1.7"
16 | gloo-timers = { version = "0.3.0", features = ["futures"] }
17 | 
18 | [dev-dependencies]
19 | wasm-bindgen = "0.2.93"
20 | wasm-bindgen-test = "0.3.42"
21 | web-sys = "0.3.70"
22 | 


--------------------------------------------------------------------------------
/examples/counter/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/wasm-test.toml" },
4 |     { path = "../cargo-make/trunk_server.toml" },
5 | ]
6 | 


--------------------------------------------------------------------------------
/examples/counter/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Counter Example
 2 | 
 3 | This example creates a simple counter in a client side rendered app with Rust and WASM!
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/counter/index.html:
--------------------------------------------------------------------------------
 1 | <!doctype html>
 2 | <html>
 3 |   <head>
 4 |     <link data-trunk rel="rust" data-wasm-opt="z" />
 5 |     <link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico" />
 6 |   </head>
 7 |   <body></body>
 8 | </html>
 9 | 
10 | 


--------------------------------------------------------------------------------
/examples/counter/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/counter/public/favicon.ico


--------------------------------------------------------------------------------
/examples/counter/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/counter/src/main.rs:
--------------------------------------------------------------------------------
 1 | use counter::SimpleCounter;
 2 | use leptos::prelude::*;
 3 | 
 4 | pub fn main() {
 5 |     _ = console_log::init_with_level(log::Level::Debug);
 6 |     console_error_panic_hook::set_once();
 7 |     mount_to_body(|| {
 8 |         view! { <SimpleCounter initial_value=0 step=1/> }
 9 |     })
10 | }
11 | 


--------------------------------------------------------------------------------
/examples/counter_isomorphic/.gitignore:
--------------------------------------------------------------------------------
1 | .leptos.kdl


--------------------------------------------------------------------------------
/examples/counter_isomorphic/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "counter_isomorphic"
9 | 


--------------------------------------------------------------------------------
/examples/counter_isomorphic/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Counter Isomorphic Example
 2 | 
 3 | This example demonstrates how to use a function isomorphically, to run a server side function from the browser and receive a result.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `cargo leptos watch` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/counter_isomorphic/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/counter_isomorphic/public/favicon.ico


--------------------------------------------------------------------------------
/examples/counter_isomorphic/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod counters;
 2 | 
 3 | #[cfg(feature = "hydrate")]
 4 | #[wasm_bindgen::prelude::wasm_bindgen]
 5 | pub fn hydrate() {
 6 |     use crate::counters::Counters;
 7 | 
 8 |     _ = console_log::init_with_level(log::Level::Debug);
 9 |     console_error_panic_hook::set_once();
10 | 
11 |     leptos::mount::hydrate_body(Counters);
12 | }
13 | 


--------------------------------------------------------------------------------
/examples/counter_url_query/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "counter_url_query"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [profile.release]
 7 | codegen-units = 1
 8 | lto = true
 9 | 
10 | [dependencies]
11 | leptos = { path = "../../leptos", features = ["csr"] }
12 | leptos_router = { path = "../../router", features = [] }
13 | console_error_panic_hook = "0.1.7"
14 | 
15 | [dev-dependencies]
16 | wasm-bindgen = "0.2.93"
17 | wasm-bindgen-test = "0.3.42"
18 | web-sys = "0.3.70"
19 | 


--------------------------------------------------------------------------------
/examples/counter_url_query/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/trunk_server.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/examples/counter_url_query/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Query Counter Example
 2 | 
 3 | This example creates a simple counter whose state is persisted and synced in the url with query params.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/counter_url_query/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>


--------------------------------------------------------------------------------
/examples/counter_url_query/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/counter_url_query/public/favicon.ico


--------------------------------------------------------------------------------
/examples/counter_url_query/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/counter_url_query/src/main.rs:
--------------------------------------------------------------------------------
 1 | use counter_url_query::SimpleQueryCounter;
 2 | use leptos::prelude::*;
 3 | use leptos_router::components::Router;
 4 | 
 5 | pub fn main() {
 6 |     console_error_panic_hook::set_once();
 7 |     leptos::mount::mount_to_body(|| {
 8 |         view! {
 9 |             <Router>
10 |                 <SimpleQueryCounter/>
11 |             </Router>
12 |         }
13 |     })
14 | }
15 | 


--------------------------------------------------------------------------------
/examples/counter_without_macros/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = [
 2 |   { path = "../cargo-make/main.toml" },
 3 |   { path = "../cargo-make/wasm-test.toml" },
 4 |   { path = "../cargo-make/trunk_server.toml" },
 5 | ]
 6 | 
 7 | [tasks.build]
 8 | dependencies = ["cargo-all-features"]
 9 | command = "cargo"
10 | args = ["all-features", "build"]
11 | 
12 | [tasks.check]
13 | dependencies = ["cargo-all-features"]
14 | command = "cargo"
15 | args = ["all-features", "clippy"]
16 | 


--------------------------------------------------------------------------------
/examples/counter_without_macros/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Counter Example
 2 | 
 3 | This example is the same like the `counter` but it's written without using macros and can be build with stable Rust.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/counter_without_macros/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>


--------------------------------------------------------------------------------
/examples/counter_without_macros/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/counter_without_macros/public/favicon.ico


--------------------------------------------------------------------------------
/examples/counter_without_macros/src/main.rs:
--------------------------------------------------------------------------------
1 | use counter_without_macros::counter;
2 | 
3 | /// Show the counter
4 | pub fn main() {
5 |     console_error_panic_hook::set_once();
6 |     leptos::mount::mount_to_body(|| counter(0, 1))
7 | }
8 | 


--------------------------------------------------------------------------------
/examples/counters/.gitignore:
--------------------------------------------------------------------------------
1 | # Support playwright testing
2 | node_modules/
3 | test-results/
4 | end2end/playwright-report/
5 | playwright/.cache/
6 | pnpm-lock.yaml


--------------------------------------------------------------------------------
/examples/counters/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "counters"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dependencies]
 7 | leptos = { path = "../../leptos", features = ["csr"] }
 8 | console_error_panic_hook = "0.1.7"
 9 | 
10 | [dev-dependencies]
11 | wasm-bindgen-test = "0.3.42"
12 | wasm-bindgen = "0.2.93"
13 | web-sys = "0.3.70"
14 | 


--------------------------------------------------------------------------------
/examples/counters/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/wasm-test.toml" },
4 |     { path = "../cargo-make/trunk_server.toml" },
5 |     { path = "../cargo-make/playwright-trunk-test.toml" },
6 | ]
7 | 


--------------------------------------------------------------------------------
/examples/counters/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Counters Example
 2 | 
 3 | This example showcases a basic leptos app with many counters. It is a good example of how to setup a basic reactive app with signals and effects, and how to interact with browser events.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/counters/e2e/.gitignore:
--------------------------------------------------------------------------------
1 | node_modules/
2 | /test-results/
3 | /playwright-report/
4 | /playwright/.cache/
5 | 


--------------------------------------------------------------------------------
/examples/counters/e2e/package.json:
--------------------------------------------------------------------------------
 1 | {
 2 |   "private": "true",
 3 |   "scripts": {},
 4 |   "devDependencies": {
 5 |     "@playwright/test": "^1.46.1"
 6 |   },
 7 |   "dependencies": {
 8 |     "pnpm": "^9.7.1"
 9 |   }
10 | }
11 | 


--------------------------------------------------------------------------------
/examples/counters/e2e/tests/add_counter.spec.ts:
--------------------------------------------------------------------------------
 1 | import { test, expect } from "@playwright/test";
 2 | import { CountersPage } from "./fixtures/counters_page";
 3 | 
 4 | test.describe("Add Counter", () => {
 5 |   test("should increase the number of counters", async ({ page }) => {
 6 |     const ui = new CountersPage(page);
 7 |     await ui.goto();
 8 | 
 9 |     await ui.addCounter();
10 |     await ui.addCounter();
11 |     await ui.addCounter();
12 | 
13 |     await expect(ui.counters).toHaveText("3");
14 |   });
15 | });
16 | 


--------------------------------------------------------------------------------
/examples/counters/e2e/tests/decrement_count.spec.ts:
--------------------------------------------------------------------------------
 1 | import { test, expect } from "@playwright/test";
 2 | import { CountersPage } from "./fixtures/counters_page";
 3 | 
 4 | test.describe("Decrement Count", () => {
 5 |   test("should decrease the total count", async ({ page }) => {
 6 |     const ui = new CountersPage(page);
 7 |     await ui.goto();
 8 |     await ui.addCounter();
 9 | 
10 |     await ui.decrementCount();
11 |     await ui.decrementCount();
12 |     await ui.decrementCount();
13 | 
14 |     await expect(ui.total).toHaveText("-3");
15 |   });
16 | });
17 | 


--------------------------------------------------------------------------------
/examples/counters/e2e/tests/increment_count.spec.ts:
--------------------------------------------------------------------------------
 1 | import { test, expect } from "@playwright/test";
 2 | import { CountersPage } from "./fixtures/counters_page";
 3 | 
 4 | test.describe("Increment Count", () => {
 5 |   test("should increase the total count", async ({ page }) => {
 6 |     const ui = new CountersPage(page);
 7 |     await ui.goto();
 8 |     await ui.addCounter();
 9 | 
10 |     await ui.incrementCount();
11 |     await ui.incrementCount();
12 |     await ui.incrementCount();
13 | 
14 |     await expect(ui.total).toHaveText("3");
15 |   });
16 | });
17 | 


--------------------------------------------------------------------------------
/examples/counters/e2e/tests/remove_counter.spec.ts:
--------------------------------------------------------------------------------
 1 | import { test, expect } from "@playwright/test";
 2 | import { CountersPage } from "./fixtures/counters_page";
 3 | 
 4 | test.describe("Remove Counter", () => {
 5 |   test("should decrement the number of counters", async ({ page }) => {
 6 |     const ui = new CountersPage(page);
 7 |     await ui.goto();
 8 | 
 9 |     await ui.addCounter();
10 |     await ui.addCounter();
11 |     await ui.addCounter();
12 | 
13 |     await ui.removeCounter(1);
14 | 
15 |     await expect(ui.counters).toHaveText("2");
16 |   });
17 | });
18 | 


--------------------------------------------------------------------------------
/examples/counters/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z" data-weak-refs/>
5 | 		<title>Counters</title>
6 | 	</head>
7 | 	<body></body>
8 | </html>
9 | 


--------------------------------------------------------------------------------
/examples/counters/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/counters/src/main.rs:
--------------------------------------------------------------------------------
1 | use counters::Counters;
2 | 
3 | fn main() {
4 |     console_error_panic_hook::set_once();
5 |     leptos::mount::mount_to_body(Counters)
6 | }
7 | 


--------------------------------------------------------------------------------
/examples/directives/.cargo/config.toml:
--------------------------------------------------------------------------------
1 | [build]
2 | rustflags = ["--cfg=web_sys_unstable_apis"]
3 | 


--------------------------------------------------------------------------------
/examples/directives/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "directives"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dependencies]
 7 | leptos = { path = "../../leptos", features = ["csr"] }
 8 | log = "0.4.22"
 9 | console_log = "1.0"
10 | console_error_panic_hook = "0.1.7"
11 | web-sys = { version = "0.3.70", features = ["Clipboard", "Navigator"] }
12 | 
13 | [dev-dependencies]
14 | wasm-bindgen-test = "0.3.42"
15 | wasm-bindgen = "0.2.93"
16 | web-sys = { version = "0.3.70", features = ["NodeList"] }


--------------------------------------------------------------------------------
/examples/directives/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/wasm-test.toml" },
4 |     { path = "../cargo-make/trunk_server.toml" },
5 | ]
6 | 


--------------------------------------------------------------------------------
/examples/directives/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Directives Example
 2 | 
 3 | This example showcases a basic leptos app that shows how to write and use directives.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/directives/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z" data-weak-refs/>
5 | 	</head>
6 | 	<body></body>
7 | </html>


--------------------------------------------------------------------------------
/examples/directives/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/directives/src/main.rs:
--------------------------------------------------------------------------------
1 | use directives::App;
2 | use leptos::prelude::*;
3 | 
4 | fn main() {
5 |     _ = console_log::init_with_level(log::Level::Debug);
6 |     console_error_panic_hook::set_once();
7 |     mount_to_body(|| view! { <App/> })
8 | }
9 | 


--------------------------------------------------------------------------------
/examples/error_boundary/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | 
 5 | # Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
 6 | # More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
 7 | Cargo.lock
 8 | 
 9 | # These are backup files generated by rustfmt
10 | **/*.rs.bk
11 | 
12 | # Support playwright testing
13 | node_modules/
14 | test-results/
15 | end2end/playwright-report/
16 | playwright/.cache/
17 | pnpm-lock.yaml
18 | 
19 | # Support trunk
20 | dist
21 | 


--------------------------------------------------------------------------------
/examples/error_boundary/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "error_boundary"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [profile.release]
 7 | codegen-units = 1
 8 | lto = true
 9 | 
10 | [dependencies]
11 | leptos = { path = "../../leptos", features = ["csr"] }
12 | console_log = "1.0"
13 | log = "0.4.22"
14 | console_error_panic_hook = "0.1.7"
15 | 


--------------------------------------------------------------------------------
/examples/error_boundary/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/playwright-trunk-test.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/examples/error_boundary/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos `<ErrorBoundary/>` Example
 2 | 
 3 | This example shows how to handle basic errors using Leptos.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Testing
10 | 
11 | This project is configured to run start and stop of processes for integration tests without the use of Cargo Leptos or Node.
12 | 
13 | ## Quick Start
14 | 
15 | Run `trunk serve --open` to run this example.
16 | 


--------------------------------------------------------------------------------
/examples/error_boundary/e2e/.gitignore:
--------------------------------------------------------------------------------
1 | node_modules/
2 | /test-results/
3 | /playwright-report/
4 | /playwright/.cache/
5 | 


--------------------------------------------------------------------------------
/examples/error_boundary/e2e/package.json:
--------------------------------------------------------------------------------
1 | {
2 |   "private": "true",
3 |   "scripts": {},
4 |   "devDependencies": {
5 |     "@playwright/test": "^1.35.1"
6 |   }
7 | }
8 | 


--------------------------------------------------------------------------------
/examples/error_boundary/e2e/tests/click_up_arrow.spec.ts:
--------------------------------------------------------------------------------
 1 | import { test, expect } from "@playwright/test";
 2 | import { HomePage } from "./fixtures/home_page";
 3 | 
 4 | test.describe("Click Up Arrow", () => {
 5 |   test("should see the positive number", async ({ page }) => {
 6 |     const ui = new HomePage(page);
 7 |     await ui.goto();
 8 | 
 9 |     await ui.clickUpArrow();
10 |     await ui.clickUpArrow();
11 |     await ui.clickUpArrow();
12 | 
13 |     await expect(ui.successMessage).toHaveText("You entered 3");
14 |   });
15 | });
16 | 


--------------------------------------------------------------------------------
/examples/error_boundary/e2e/tests/open_app.spec.ts:
--------------------------------------------------------------------------------
 1 | import { test, expect } from "@playwright/test";
 2 | import { HomePage } from "./fixtures/home_page";
 3 | 
 4 | test.describe("Open App", () => {
 5 |   test("should see the page title", async ({ page }) => {
 6 |     const ui = new HomePage(page);
 7 |     await ui.goto();
 8 | 
 9 |     await expect(ui.pageTitle).toHaveText("Error Handling");
10 |   });
11 | });
12 | 


--------------------------------------------------------------------------------
/examples/error_boundary/e2e/tests/type_number.spec.ts:
--------------------------------------------------------------------------------
 1 | import { test, expect } from "@playwright/test";
 2 | import { HomePage } from "./fixtures/home_page";
 3 | 
 4 | test.describe("Type Number", () => {
 5 |   test("should see the typed number", async ({ page }) => {
 6 |     const ui = new HomePage(page);
 7 |     await ui.goto();
 8 | 
 9 |     await ui.enterNumber("7");
10 | 
11 |     await expect(ui.successMessage).toHaveText("You entered 7");
12 |   });
13 | });
14 | 


--------------------------------------------------------------------------------
/examples/error_boundary/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>


--------------------------------------------------------------------------------
/examples/error_boundary/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/error_boundary/public/favicon.ico


--------------------------------------------------------------------------------
/examples/error_boundary/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/error_boundary/src/main.rs:
--------------------------------------------------------------------------------
1 | use error_boundary::*;
2 | use leptos::prelude::*;
3 | 
4 | pub fn main() {
5 |     _ = console_log::init_with_level(log::Level::Debug);
6 |     console_error_panic_hook::set_once();
7 |     mount_to_body(App)
8 | }
9 | 


--------------------------------------------------------------------------------
/examples/errors_axum/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "errors_axum"
9 | 


--------------------------------------------------------------------------------
/examples/errors_axum/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Errors Demonstration with Axum
 2 | 
 3 | This example demonstrates how Leptos Errors can work with an Axum backend on a server.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `cargo leptos watch` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/errors_axum/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/errors_axum/public/favicon.ico


--------------------------------------------------------------------------------
/examples/errors_axum/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/errors_axum/src/errors.rs:
--------------------------------------------------------------------------------
 1 | use http::status::StatusCode;
 2 | use thiserror::Error;
 3 | 
 4 | #[derive(Debug, Clone, PartialEq, Eq, Error)]
 5 | pub enum AppError {
 6 |     #[error("Not Found")]
 7 |     NotFound,
 8 |     #[error("Internal Server Error")]
 9 |     InternalServerError,
10 | }
11 | 
12 | impl AppError {
13 |     pub fn status_code(&self) -> StatusCode {
14 |         match self {
15 |             AppError::NotFound => StatusCode::NOT_FOUND,
16 |             AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
17 |         }
18 |     }
19 | }
20 | 


--------------------------------------------------------------------------------
/examples/errors_axum/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod error_template;
 2 | pub mod errors;
 3 | pub mod landing;
 4 | 
 5 | #[cfg(feature = "hydrate")]
 6 | #[wasm_bindgen::prelude::wasm_bindgen]
 7 | pub fn hydrate() {
 8 |     use crate::landing::App;
 9 |     console_error_panic_hook::set_once();
10 |     leptos::mount::hydrate_body(App);
11 | }
12 | 


--------------------------------------------------------------------------------
/examples/errors_axum/style.css:
--------------------------------------------------------------------------------
1 | .pending {
2 | 	color: purple;
3 | }


--------------------------------------------------------------------------------
/examples/fetch/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/trunk_server.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/examples/fetch/README.md:
--------------------------------------------------------------------------------
 1 | # Client Side Fetch
 2 | 
 3 | This example shows how to fetch data from the client in WebAssembly.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/fetch/index.html:
--------------------------------------------------------------------------------
 1 | <!DOCTYPE html>
 2 | <html>
 3 | 	<head>
 4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
 5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
 6 | 	</head>
 7 | 	<style>
 8 | 		img {
 9 | 			max-width: 250px;
10 | 			height: auto;
11 | 		}
12 | 
13 | 		.error {
14 | 			border: 1px solid red;
15 | 			color: red;
16 | 			background-color: lightpink;
17 | 		}
18 | 	</style>
19 | 	<body></body>
20 | </html>


--------------------------------------------------------------------------------
/examples/fetch/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/fetch/public/favicon.ico


--------------------------------------------------------------------------------
/examples/fetch/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/hackernews/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "hackernews"
9 | 


--------------------------------------------------------------------------------
/examples/hackernews/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Hacker News Example
 2 | 
 3 | This example creates a basic clone of the Hacker News site. It showcases Leptos' ability to create both a client-side rendered app, and a server side rendered app with hydration, in a single repository
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` or `cargo leptos watch` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/hackernews/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z" data-cargo-features="csr"/>
5 | 		<link data-trunk rel="css" href="./style.css"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>


--------------------------------------------------------------------------------
/examples/hackernews/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/hackernews/public/favicon.ico


--------------------------------------------------------------------------------
/examples/hackernews/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/hackernews/src/routes.rs:
--------------------------------------------------------------------------------
1 | pub mod nav;
2 | pub mod stories;
3 | pub mod story;
4 | pub mod users;
5 | 


--------------------------------------------------------------------------------
/examples/hackernews_axum/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "hackernews_axum"
9 | 


--------------------------------------------------------------------------------
/examples/hackernews_axum/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Hacker News Example with Axum
 2 | 
 3 | This example creates a basic clone of the Hacker News site. It showcases Leptos' ability to create both a client-side rendered app, and a server side rendered app with hydration, in a single repository. This repo differs from the main Hacker News example by using Axum as it's server.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` or `cargo leptos watch` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/hackernews_axum/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
5 | 		<link data-trunk rel="css" href="/style.css"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>


--------------------------------------------------------------------------------
/examples/hackernews_axum/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/hackernews_axum/public/favicon.ico


--------------------------------------------------------------------------------
/examples/hackernews_axum/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/hackernews_axum/src/routes.rs:
--------------------------------------------------------------------------------
1 | pub mod nav;
2 | pub mod stories;
3 | pub mod story;
4 | pub mod users;
5 | 


--------------------------------------------------------------------------------
/examples/hackernews_islands_axum/.cargo/config.wasm.toml:
--------------------------------------------------------------------------------
1 | [unstable]
2 | build-std = ["std", "panic_abort", "core", "alloc"]
3 | build-std-features = ["panic_immediate_abort"]
4 | 


--------------------------------------------------------------------------------
/examples/hackernews_islands_axum/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = [
 2 |   { path = "../cargo-make/main.toml" },
 3 |   { path = "../cargo-make/cargo-leptos-compress.toml" },
 4 | ]
 5 | 
 6 | [tasks.ci]
 7 | dependencies = [
 8 |   "prepare",
 9 |   "make-target-site-dir",
10 |   "lint",
11 |   "test-flow",
12 |   "integration-test",
13 | ]
14 | 
15 | [env]
16 | CLIENT_PROCESS_NAME = "hackernews_islands"
17 | 


--------------------------------------------------------------------------------
/examples/hackernews_islands_axum/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
5 | 		<link data-trunk rel="css" href="/style.css"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>


--------------------------------------------------------------------------------
/examples/hackernews_islands_axum/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/hackernews_islands_axum/public/favicon.ico


--------------------------------------------------------------------------------
/examples/hackernews_islands_axum/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/hackernews_islands_axum/src/routes.rs:
--------------------------------------------------------------------------------
1 | pub mod nav;
2 | pub mod stories;
3 | pub mod story;
4 | pub mod users;
5 | 


--------------------------------------------------------------------------------
/examples/hackernews_js_fetch/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |   { path = "../cargo-make/main.toml" },
3 |   { path = "../cargo-make/deno-build.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "deno"
9 | 


--------------------------------------------------------------------------------
/examples/hackernews_js_fetch/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Hacker News Example with Axum
 2 | 
 3 | This example uses the basic Hacker News example as its basis, but shows how to run the server side as WASM running in a JS environment. In this example, Deno is used as the runtime.
 4 | 
 5 | ## Server Side Rendering with Deno
 6 | 
 7 | To run the Deno version, run
 8 | 
 9 | ```bash
10 | deno task build
11 | deno task start
12 | ```
13 | 


--------------------------------------------------------------------------------
/examples/hackernews_js_fetch/deno.jsonc:
--------------------------------------------------------------------------------
1 | {
2 |   "tasks": {
3 |     "build:server": "wasm-pack build --release --target web --out-name server --features ssr --no-default-features",
4 |     "build:client": "wasm-pack build --release --target web --out-name client --features hydrate --no-default-features",
5 |     "build": "deno task build:server & deno task build:client",
6 |     "start": "deno run --allow-read --allow-net run.ts"
7 |   }
8 | }
9 | 


--------------------------------------------------------------------------------
/examples/hackernews_js_fetch/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/hackernews_js_fetch/public/favicon.ico


--------------------------------------------------------------------------------
/examples/hackernews_js_fetch/run.ts:
--------------------------------------------------------------------------------
 1 | import init, { Handler } from "./pkg/server.js";
 2 | import { serveDir } from "https://deno.land/std/http/file_server.ts";
 3 | 
 4 | await init();
 5 | const handler = await Handler.new();
 6 | 
 7 | Deno.serve((req) => {
 8 |   const u = new URL(req.url);
 9 |   if (u.pathname.startsWith("/pkg") || u.pathname.startsWith("/public")) {
10 |     return serveDir(req);
11 |   }
12 |   return handler.serve(req);
13 | });
14 | 


--------------------------------------------------------------------------------
/examples/hackernews_js_fetch/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/hackernews_js_fetch/src/routes.rs:
--------------------------------------------------------------------------------
1 | pub mod nav;
2 | pub mod stories;
3 | pub mod story;
4 | pub mod users;
5 | 


--------------------------------------------------------------------------------
/examples/islands/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/examples/islands/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/islands/public/favicon.ico


--------------------------------------------------------------------------------
/examples/islands/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/islands/src/lib.rs:
--------------------------------------------------------------------------------
1 | pub mod app;
2 | 
3 | #[cfg(feature = "hydrate")]
4 | #[wasm_bindgen::prelude::wasm_bindgen]
5 | pub fn hydrate() {
6 |     console_error_panic_hook::set_once();
7 |     leptos::mount::hydrate_islands();
8 | }
9 | 


--------------------------------------------------------------------------------
/examples/islands/style.css:
--------------------------------------------------------------------------------
1 | .pending {
2 | 	color: purple;
3 | }
4 | 


--------------------------------------------------------------------------------
/examples/islands_router/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/examples/islands_router/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/islands_router/public/favicon.ico


--------------------------------------------------------------------------------
/examples/islands_router/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/islands_router/src/lib.rs:
--------------------------------------------------------------------------------
1 | pub mod app;
2 | 
3 | #[cfg(feature = "hydrate")]
4 | #[wasm_bindgen::prelude::wasm_bindgen]
5 | pub fn hydrate() {
6 |     console_error_panic_hook::set_once();
7 |     leptos::mount::hydrate_islands();
8 | }
9 | 


--------------------------------------------------------------------------------
/examples/js-framework-benchmark/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = [
 2 |   { path = "../cargo-make/main.toml" },
 3 |   { path = "../cargo-make/wasm-test.toml" },
 4 |   { path = "../cargo-make/trunk_server.toml" },
 5 | ]
 6 | 
 7 | [tasks.clippy-each-feature]
 8 | dependencies = ["cargo-all-features"]
 9 | command = "cargo"
10 | args = [
11 |   "all-features",
12 |   "clippy",
13 |   "--target",
14 |   "wasm32-unknown-unknown",
15 |   "--no-deps",
16 |   "--",
17 |   "-D",
18 |   "warnings",
19 | ]
20 | 


--------------------------------------------------------------------------------
/examples/js-framework-benchmark/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos benchmark example
 2 | 
 3 | This example is adoptation of code from [js-framework-benchmark](https://github.com/krausest/js-framework-benchmark/tree/master/frameworks/keyed/leptos).
 4 | This example creates a large table with randomized entries, it also shows usage of `template` macro and `For` component.
 5 | 
 6 | ## Getting Started
 7 | 
 8 | See the [Examples README](../README.md) for setup and run instructions.
 9 | 
10 | ## Quick Start
11 | 
12 | Run `trunk serve --open` to run this example.
13 | 


--------------------------------------------------------------------------------
/examples/js-framework-benchmark/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/js-framework-benchmark/public/favicon.ico


--------------------------------------------------------------------------------
/examples/js-framework-benchmark/src/main.rs:
--------------------------------------------------------------------------------
 1 | use js_framework_benchmark_leptos::App;
 2 | use leptos::{
 3 |     leptos_dom::helpers::document, mount::mount_to, wasm_bindgen::JsCast,
 4 | };
 5 | 
 6 | pub fn main() {
 7 |     console_error_panic_hook::set_once();
 8 |     let root = document().query_selector("#main").unwrap().unwrap();
 9 |     let handle = mount_to(root.unchecked_into(), App);
10 |     handle.forget();
11 | }
12 | 


--------------------------------------------------------------------------------
/examples/parent_child/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "parent-child"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [profile.release]
 7 | codegen-units = 1
 8 | lto = true
 9 | 
10 | [dependencies]
11 | leptos = { path = "../../leptos", features = ["csr"] }
12 | console_log = "1.0"
13 | log = "0.4.22"
14 | console_error_panic_hook = "0.1.7"
15 | web-sys = "0.3.70"
16 | 


--------------------------------------------------------------------------------
/examples/parent_child/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/trunk_server.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/examples/parent_child/index.html:
--------------------------------------------------------------------------------
 1 | <!DOCTYPE html>
 2 | <html>
 3 | 	<head>
 4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
 5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
 6 | 		<style>
 7 | 			.red {
 8 | 				color: red;
 9 | 			}
10 | 
11 | 			.right {
12 | 				text-align: right;
13 | 			}
14 | 
15 | 			.italics {
16 | 				font-style: italic;
17 | 			}
18 | 
19 | 			.smallcaps {
20 | 				font-variant: small-caps;
21 | 			}
22 | 		</style>
23 | 	</head>
24 | 	<body></body>
25 | </html>


--------------------------------------------------------------------------------
/examples/parent_child/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/parent_child/public/favicon.ico


--------------------------------------------------------------------------------
/examples/parent_child/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/parent_child/src/main.rs:
--------------------------------------------------------------------------------
1 | use leptos::prelude::*;
2 | use parent_child::*;
3 | 
4 | pub fn main() {
5 |     _ = console_log::init_with_level(log::Level::Debug);
6 |     console_error_panic_hook::set_once();
7 |     mount_to_body(App)
8 | }
9 | 


--------------------------------------------------------------------------------
/examples/portal/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "portal"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dependencies]
 7 | leptos = { path = "../../leptos", features = ["csr"] }
 8 | log = "0.4.27"
 9 | console_log = "1.0"
10 | console_error_panic_hook = "0.1.7"
11 | wasm-bindgen = "0.2.100"
12 | 
13 | [dev-dependencies]
14 | wasm-bindgen-test = "0.3.50"
15 | wasm-bindgen = "0.2.100"
16 | web-sys = "0.3.77"
17 | 


--------------------------------------------------------------------------------
/examples/portal/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |   { path = "../cargo-make/main.toml" },
3 |   { path = "../cargo-make/webdriver.toml" },
4 |   { path = "../cargo-make/wasm-test.toml" },
5 |   { path = "../cargo-make/trunk_server.toml" },
6 | ]
7 | 


--------------------------------------------------------------------------------
/examples/portal/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Portal Example
 2 | 
 3 | This example showcases a basic leptos app with a portal.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/portal/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z" data-weak-refs/>
5 | 	</head>
6 | 	<body>
7 | 		<div id="app"></div>
8 | 	</body>
9 | </html>


--------------------------------------------------------------------------------
/examples/portal/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/portal/src/main.rs:
--------------------------------------------------------------------------------
 1 | use leptos::prelude::*;
 2 | use portal::App;
 3 | use wasm_bindgen::JsCast;
 4 | 
 5 | fn main() {
 6 |     _ = console_log::init_with_level(log::Level::Debug);
 7 |     console_error_panic_hook::set_once();
 8 |     let handle = mount_to(
 9 |         document()
10 |             .get_element_by_id("app")
11 |             .unwrap()
12 |             .unchecked_into(),
13 |         App,
14 |     );
15 |     handle.forget();
16 | }
17 | 


--------------------------------------------------------------------------------
/examples/router/.cargo/config.toml:
--------------------------------------------------------------------------------
1 | [target.wasm32-unknown-unknown]
2 | rustflags = ["-C", "panic=abort"]
3 | 


--------------------------------------------------------------------------------
/examples/router/.gitignore:
--------------------------------------------------------------------------------
 1 |  Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | 
 5 | # Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
 6 | # More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
 7 | Cargo.lock
 8 | 
 9 | # These are backup files generated by rustfmt
10 | **/*.rs.bk
11 | 
12 | # Support playwright testing
13 | node_modules/
14 | test-results/
15 | end2end/playwright-report/
16 | playwright/.cache/
17 | pnpm-lock.yaml
18 | 
19 | # Support trunk
20 | dist
21 | 
22 | 


--------------------------------------------------------------------------------
/examples/router/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |   { path = "../cargo-make/main.toml" },
3 |   { path = "../cargo-make/trunk_server.toml" },
4 |   { path = "../cargo-make/playwright-test.toml" },
5 | ]
6 | 


--------------------------------------------------------------------------------
/examples/router/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Router Example
 2 | 
 3 | This example demonstrates how Leptos’s router works for client side routing.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/router/e2e/package.json:
--------------------------------------------------------------------------------
1 | {
2 |   "private": "true",
3 |   "scripts": {},
4 |   "devDependencies": {
5 |     "@playwright/test": "^1.35.1"
6 |   }
7 | }
8 | 


--------------------------------------------------------------------------------
/examples/router/index.html:
--------------------------------------------------------------------------------
 1 | <!DOCTYPE html>
 2 | <html>
 3 | 	<head>
 4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
 5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
 6 | 		<link data-trunk rel="css" href="style.css"/>
 7 | 	</head>
 8 | 	<body></body>
 9 | </html>
10 | 


--------------------------------------------------------------------------------
/examples/router/package.json:
--------------------------------------------------------------------------------
 1 | {
 2 |   "private": true,
 3 |   "scripts": {
 4 |     "start-server": "trunk serve",
 5 |     "e2e": "cargo make test-playwright",
 6 |     "e2e:auto-start": "start-server-and-test start-server http://127.0.0.1:8080 e2e"
 7 |   },
 8 |   "devDependencies": {
 9 |     "start-server-and-test": "^2.0.0"
10 |   }
11 | }
12 | 


--------------------------------------------------------------------------------
/examples/router/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/router/public/favicon.ico


--------------------------------------------------------------------------------
/examples/router/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | components = ["rust-src"]
4 | 


--------------------------------------------------------------------------------
/examples/router/src/main.rs:
--------------------------------------------------------------------------------
 1 | use leptos::prelude::*;
 2 | use router::*;
 3 | use tracing_subscriber::fmt;
 4 | use tracing_subscriber_wasm::MakeConsoleWriter;
 5 | 
 6 | pub fn main() {
 7 |     fmt()
 8 |         .with_writer(
 9 |             MakeConsoleWriter::default()
10 |                 .map_trace_level_to(tracing::Level::DEBUG),
11 |         )
12 |         .without_time()
13 |         .init();
14 |     console_error_panic_hook::set_once();
15 |     mount_to_body(RouterExample);
16 | }
17 | 


--------------------------------------------------------------------------------
/examples/server_fns_axum/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "server_fns_axum"
9 | 


--------------------------------------------------------------------------------
/examples/server_fns_axum/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/server_fns_axum/public/favicon.ico


--------------------------------------------------------------------------------
/examples/server_fns_axum/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/server_fns_axum/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod app;
 2 | pub mod error_template;
 3 | pub mod errors;
 4 | #[cfg(feature = "ssr")]
 5 | pub mod middleware;
 6 | 
 7 | #[cfg(feature = "hydrate")]
 8 | #[wasm_bindgen::prelude::wasm_bindgen]
 9 | pub fn hydrate() {
10 |     use crate::app::App;
11 |     console_error_panic_hook::set_once();
12 |     leptos::mount::hydrate_body(App);
13 | }
14 | 


--------------------------------------------------------------------------------
/examples/server_fns_axum/style.css:
--------------------------------------------------------------------------------
1 | .pending {
2 | 	color: purple;
3 | }
4 | 


--------------------------------------------------------------------------------
/examples/server_fns_axum/watched_files/.gitkeep:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/server_fns_axum/watched_files/.gitkeep


--------------------------------------------------------------------------------
/examples/slots/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "slots"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [profile.release]
 7 | codegen-units = 1
 8 | lto = true
 9 | 
10 | [dependencies]
11 | leptos = { path = "../../leptos", features = ["csr"] }
12 | console_log = "1.0"
13 | log = "0.4.22"
14 | console_error_panic_hook = "0.1.7"
15 | 


--------------------------------------------------------------------------------
/examples/slots/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/trunk_server.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/examples/slots/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos `<Component slot/>` Example
 2 | 
 3 | This example shows how to use Slots in Leptos.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/slots/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>


--------------------------------------------------------------------------------
/examples/slots/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/slots/public/favicon.ico


--------------------------------------------------------------------------------
/examples/slots/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/slots/src/main.rs:
--------------------------------------------------------------------------------
1 | use slots::App;
2 | 
3 | pub fn main() {
4 |     _ = console_log::init_with_level(log::Level::Debug);
5 |     console_error_panic_hook::set_once();
6 |     leptos::mount::mount_to_body(App);
7 | }
8 | 


--------------------------------------------------------------------------------
/examples/spread/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "spread"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [profile.release]
 7 | codegen-units = 1
 8 | lto = true
 9 | 
10 | [dependencies]
11 | leptos = { path = "../../leptos", features = ["csr"] }
12 | console_error_panic_hook = "0.1.7"
13 | 


--------------------------------------------------------------------------------
/examples/spread/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/trunk_server.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/examples/spread/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Attribute and EventHandler spreading Example
 2 | 
 3 | This example creates a simple element in a client side rendered app with Rust and WASM!
 4 | 
 5 | Dynamic sets of attributes and event handler are spread onto the element with little effort.
 6 | 
 7 | ## Getting Started
 8 | 
 9 | See the [Examples README](../README.md) for setup and run instructions.
10 | 
11 | ## Quick Start
12 | 
13 | Run `trunk serve --open` to run this example.
14 | 


--------------------------------------------------------------------------------
/examples/spread/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>


--------------------------------------------------------------------------------
/examples/spread/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/spread/public/favicon.ico


--------------------------------------------------------------------------------
/examples/spread/src/main.rs:
--------------------------------------------------------------------------------
1 | use spread::SpreadingExample;
2 | 
3 | pub fn main() {
4 |     console_error_panic_hook::set_once();
5 |     leptos::mount::mount_to_body(SpreadingExample)
6 | }
7 | 


--------------------------------------------------------------------------------
/examples/ssr_modes/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | pkg
 5 | 
 6 | # These are backup files generated by rustfmt
 7 | **/*.rs.bk
 8 | 
 9 | # node e2e test tools and outputs
10 | node_modules/
11 | test-results/
12 | end2end/playwright-report/
13 | playwright/.cache/
14 | 


--------------------------------------------------------------------------------
/examples/ssr_modes/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "ssr_modes"
9 | 


--------------------------------------------------------------------------------
/examples/ssr_modes/assets/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/ssr_modes/assets/favicon.ico


--------------------------------------------------------------------------------
/examples/ssr_modes/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/ssr_modes/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod app;
 2 | 
 3 | #[cfg(feature = "hydrate")]
 4 | #[wasm_bindgen::prelude::wasm_bindgen]
 5 | pub fn hydrate() {
 6 |     use app::*;
 7 | 
 8 |     // initializes logging using the `log` crate
 9 |     _ = console_log::init_with_level(log::Level::Debug);
10 |     console_error_panic_hook::set_once();
11 | 
12 |     leptos::mount::hydrate_body(App);
13 | }
14 | 


--------------------------------------------------------------------------------
/examples/ssr_modes/style/main.scss:
--------------------------------------------------------------------------------
1 | body {
2 | 	font-family: sans-serif;
3 | }
4 | 


--------------------------------------------------------------------------------
/examples/ssr_modes_axum/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | pkg
 5 | 
 6 | # These are backup files generated by rustfmt
 7 | **/*.rs.bk
 8 | 
 9 | # node e2e test tools and outputs
10 | node_modules/
11 | test-results/
12 | end2end/playwright-report/
13 | playwright/.cache/
14 | 


--------------------------------------------------------------------------------
/examples/ssr_modes_axum/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "ssr_modes_axum"
9 | 


--------------------------------------------------------------------------------
/examples/ssr_modes_axum/assets/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/ssr_modes_axum/assets/favicon.ico


--------------------------------------------------------------------------------
/examples/ssr_modes_axum/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/ssr_modes_axum/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod app;
 2 | 
 3 | #[cfg(feature = "hydrate")]
 4 | #[wasm_bindgen::prelude::wasm_bindgen]
 5 | pub fn hydrate() {
 6 |     use app::*;
 7 |     console_error_panic_hook::set_once();
 8 |     leptos::mount::hydrate_body(App);
 9 | }
10 | 


--------------------------------------------------------------------------------
/examples/ssr_modes_axum/style/main.scss:
--------------------------------------------------------------------------------
1 | body {
2 | 	font-family: sans-serif;
3 | }


--------------------------------------------------------------------------------
/examples/static_routing/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | pkg
 5 | 
 6 | # These are backup files generated by rustfmt
 7 | **/*.rs.bk
 8 | 
 9 | # node e2e test tools and outputs
10 | node_modules/
11 | test-results/
12 | end2end/playwright-report/
13 | playwright/.cache/
14 | 


--------------------------------------------------------------------------------
/examples/static_routing/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "ssr_modes_axum"
9 | 


--------------------------------------------------------------------------------
/examples/static_routing/README.md:
--------------------------------------------------------------------------------
 1 | # Static Routing Example
 2 | 
 3 | This example shows the static routing features, which can be used to generate the HTML content for some routes before a request.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `cargo leptos watch` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/static_routing/assets/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/static_routing/assets/favicon.ico


--------------------------------------------------------------------------------
/examples/static_routing/posts/post1.md:
--------------------------------------------------------------------------------
1 | # My first blog post
2 | 
3 | Having a blog is *fun*.
4 | 


--------------------------------------------------------------------------------
/examples/static_routing/posts/post2.md:
--------------------------------------------------------------------------------
1 | # My second blog post
2 | 
3 | Coming up with content is hard.
4 | 


--------------------------------------------------------------------------------
/examples/static_routing/posts/post3.md:
--------------------------------------------------------------------------------
1 | # My third blog post
2 | 
3 | Could I just have AI write this for me instead?
4 | 


--------------------------------------------------------------------------------
/examples/static_routing/posts/post4.md:
--------------------------------------------------------------------------------
1 | # My fourth post
2 | 
3 | Here is some content. It should regenerate the static page.
4 | 


--------------------------------------------------------------------------------
/examples/static_routing/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/static_routing/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod app;
 2 | 
 3 | #[cfg(feature = "hydrate")]
 4 | #[wasm_bindgen::prelude::wasm_bindgen]
 5 | pub fn hydrate() {
 6 |     use app::*;
 7 |     console_error_panic_hook::set_once();
 8 |     leptos::mount::hydrate_body(App);
 9 | }
10 | 


--------------------------------------------------------------------------------
/examples/static_routing/style/main.scss:
--------------------------------------------------------------------------------
1 | body {
2 | 	font-family: sans-serif;
3 | }


--------------------------------------------------------------------------------
/examples/stores/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 | ]
4 | 


--------------------------------------------------------------------------------
/examples/stores/README.md:
--------------------------------------------------------------------------------
 1 | # Stores Example
 2 | 
 3 | This example shows how to use reactive stores, by building a client-side rendered TODO application.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/stores/index.html:
--------------------------------------------------------------------------------
 1 | <!DOCTYPE html>
 2 | <html>
 3 | 	<head>
 4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
 5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
 6 | 		<style>
 7 | 		.hidden {
 8 | 			display: none;
 9 | 		}
10 | 		</style>
11 | 	</head>
12 | 	<body></body>
13 | </html>
14 | 


--------------------------------------------------------------------------------
/examples/stores/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/stores/public/favicon.ico


--------------------------------------------------------------------------------
/examples/stores/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/stores/src/main.rs:
--------------------------------------------------------------------------------
1 | use leptos::prelude::*;
2 | use stores::App;
3 | 
4 | pub fn main() {
5 |     console_error_panic_hook::set_once();
6 |     mount_to_body(App)
7 | }
8 | 


--------------------------------------------------------------------------------
/examples/suspense_tests/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | pkg
 5 | 
 6 | # These are backup files generated by rustfmt
 7 | **/*.rs.bk
 8 | 
 9 | # node e2e test tools and outputs
10 | node_modules/
11 | test-results/
12 | end2end/playwright-report/
13 | playwright/.cache/
14 | 


--------------------------------------------------------------------------------
/examples/suspense_tests/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = [
 2 |     { path = "../cargo-make/main.toml" },
 3 |     { path = "../cargo-make/cargo-leptos-webdriver-test.toml" },
 4 | ]
 5 | 
 6 | [env]
 7 | CLIENT_PROCESS_NAME = "suspense_tests"
 8 | 
 9 | [tasks.test-ui]
10 | cwd = "./e2e"
11 | command = "cargo"
12 | args = ["make", "test-ui", "${@}"]
13 | 


--------------------------------------------------------------------------------
/examples/suspense_tests/README.md:
--------------------------------------------------------------------------------
 1 | # Suspense Test Example
 2 | 
 3 | This example demonstrates the `<Suspense/>` behavior.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Test Strategy
10 | 
11 | See the [E2E README](./e2e/README.md) to learn about the web testing strategy for this project.
12 | 
13 | ## Quick Start
14 | 
15 | Run `cargo leptos watch` to run this example.
16 | 


--------------------------------------------------------------------------------
/examples/suspense_tests/e2e/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "suspense_tests_e2e"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dev-dependencies]
 7 | anyhow = "1.0"
 8 | async-trait = "0.1.81"
 9 | cucumber = "0.21.1"
10 | fantoccini = "0.21.1"
11 | pretty_assertions = "1.4"
12 | serde_json = "1.0"
13 | tokio = { version = "1.39", features = ["macros", "rt-multi-thread", "time"] }
14 | url = "2.5"
15 | 
16 | [[test]]
17 | name = "app_suite"
18 | harness = false    # Allow Cucumber to print output instead of libtest
19 | 


--------------------------------------------------------------------------------
/examples/suspense_tests/e2e/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = { path = "../../cargo-make/main.toml" }
 2 | 
 3 | [tasks.test]
 4 | env = { RUN_AUTOMATICALLY = false }
 5 | condition = { env_true = ["RUN_AUTOMATICALLY"] }
 6 | 
 7 | [tasks.ci]
 8 | 
 9 | [tasks.test-ui]
10 | command = "cargo"
11 | args = [
12 |   "test",
13 |   "--test",
14 |   "app_suite",
15 |   "--",
16 |   "--retry",
17 |   "2",
18 |   "--fail-fast",
19 |   "${@}",
20 | ]
21 | 


--------------------------------------------------------------------------------
/examples/suspense_tests/e2e/features/click_nested_count.feature:
--------------------------------------------------------------------------------
 1 | @click_nested_count
 2 | Feature: Click Nested Count
 3 | 
 4 |     Background:
 5 | 
 6 |         Given I see the app
 7 | 
 8 |     Scenario Outline: Should increase the count
 9 | 
10 |         Given I select the mode <Mode>
11 |         And I select the component Nested
12 |         When I click the count 3 times
13 |         Then I see the count is 3
14 | 
15 |         Examples:
16 |             | Mode         |
17 |             | Out-of-Order |
18 |             | In-Order     |
19 |             | Async        |
20 | 


--------------------------------------------------------------------------------
/examples/suspense_tests/e2e/features/click_single_count.feature:
--------------------------------------------------------------------------------
 1 | @click_single_count
 2 | Feature: Click Single Count
 3 | 
 4 |     Background:
 5 | 
 6 |         Given I see the app
 7 | 
 8 |     Scenario Outline: Should increase the count
 9 | 
10 |         Given I select the mode <Mode>
11 |         And I select the component Single
12 |         When I click the count 3 times
13 |         Then I see the count is 3
14 | 
15 |         Examples:
16 |             | Mode         |
17 |             | Out-of-Order |
18 |             | In-Order     |
19 |             | Async        |
20 | 


--------------------------------------------------------------------------------
/examples/suspense_tests/e2e/features/open_app.feature:
--------------------------------------------------------------------------------
1 | @open_app
2 | Feature: Open App
3 | 
4 |   @open_app-title
5 |   Scenario: Should see the initial page title
6 |     When I open the app
7 |     Then I see the page title is Out-of-Order


--------------------------------------------------------------------------------
/examples/suspense_tests/e2e/tests/fixtures/mod.rs:
--------------------------------------------------------------------------------
1 | pub mod action;
2 | pub mod check;
3 | pub mod find;
4 | pub mod world;
5 | 


--------------------------------------------------------------------------------
/examples/suspense_tests/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod app;
 2 | mod instrumented;
 3 | 
 4 | #[cfg(feature = "hydrate")]
 5 | #[wasm_bindgen::prelude::wasm_bindgen]
 6 | pub fn hydrate() {
 7 |     use app::*;
 8 | 
 9 |     console_error_panic_hook::set_once();
10 | 
11 |     leptos::mount::hydrate_body(App);
12 | }
13 | 


--------------------------------------------------------------------------------
/examples/tailwind_actix/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | 
 5 | # Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
 6 | # More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
 7 | Cargo.lock
 8 | 
 9 | # These are backup files generated by rustfmt
10 | **/*.rs.bk
11 | 
12 | # Support playwright testing
13 | node_modules/
14 | test-results/
15 | end2end/playwright-report/
16 | playwright/.cache/
17 | pnpm-lock.yaml
18 | 


--------------------------------------------------------------------------------
/examples/tailwind_actix/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/playwright.toml" },
4 |     { path = "../cargo-make/cargo-leptos-test.toml" },
5 | ]
6 | 
7 | [env]
8 | CLIENT_PROCESS_NAME = "tailwind"
9 | 


--------------------------------------------------------------------------------
/examples/tailwind_actix/end2end/package.json:
--------------------------------------------------------------------------------
 1 | {
 2 |   "name": "end2end",
 3 |   "version": "1.0.0",
 4 |   "description": "",
 5 |   "main": "index.js",
 6 |   "scripts": {},
 7 |   "keywords": [],
 8 |   "author": "",
 9 |   "license": "ISC",
10 |   "devDependencies": {
11 |     "@playwright/test": "^1.28.0"
12 |   }
13 | }
14 | 


--------------------------------------------------------------------------------
/examples/tailwind_actix/end2end/tests/example.spec.ts:
--------------------------------------------------------------------------------
1 | import { test, expect } from "@playwright/test";
2 | 
3 | test("homepage has title 'Leptos + Tailwindcss'", async ({ page }) => {
4 |   await page.goto("http://localhost:3000/");
5 | 
6 |   await expect(page).toHaveTitle("Leptos + Tailwindcss");
7 | });
8 | 


--------------------------------------------------------------------------------
/examples/tailwind_actix/input.css:
--------------------------------------------------------------------------------
1 | @import "tailwindcss";
2 | 


--------------------------------------------------------------------------------
/examples/tailwind_actix/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/tailwind_actix/public/favicon.ico


--------------------------------------------------------------------------------
/examples/tailwind_actix/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/tailwind_actix/src/lib.rs:
--------------------------------------------------------------------------------
 1 | mod app;
 2 | 
 3 | #[cfg(feature = "hydrate")]
 4 | #[wasm_bindgen::prelude::wasm_bindgen]
 5 | pub fn hydrate() {
 6 |     use crate::app::App;
 7 |     console_error_panic_hook::set_once();
 8 |     leptos::mount::hydrate_body(App);
 9 | }
10 | 


--------------------------------------------------------------------------------
/examples/tailwind_axum/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | 
 5 | # Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
 6 | # More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
 7 | Cargo.lock
 8 | 
 9 | # These are backup files generated by rustfmt
10 | **/*.rs.bk
11 | 
12 | # Support playwright testing
13 | node_modules/
14 | test-results/
15 | end2end/playwright-report/
16 | playwright/.cache/
17 | pnpm-lock.yaml
18 | 


--------------------------------------------------------------------------------
/examples/tailwind_axum/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "leptos-tailwind"
9 | 


--------------------------------------------------------------------------------
/examples/tailwind_axum/end2end/package.json:
--------------------------------------------------------------------------------
 1 | {
 2 |   "name": "end2end",
 3 |   "version": "1.0.0",
 4 |   "description": "",
 5 |   "main": "index.js",
 6 |   "scripts": {},
 7 |   "keywords": [],
 8 |   "author": "",
 9 |   "license": "ISC",
10 |   "devDependencies": {
11 |     "@playwright/test": "^1.28.0"
12 |   }
13 | }
14 | 


--------------------------------------------------------------------------------
/examples/tailwind_axum/end2end/tests/example.spec.ts:
--------------------------------------------------------------------------------
1 | import { test, expect } from "@playwright/test";
2 | 
3 | test("homepage has title 'Leptos + Tailwindcss'", async ({ page }) => {
4 |   await page.goto("http://localhost:3000/");
5 | 
6 |   await expect(page).toHaveTitle("Leptos + Tailwindcss");
7 | });
8 | 


--------------------------------------------------------------------------------
/examples/tailwind_axum/input.css:
--------------------------------------------------------------------------------
1 | @import "tailwindcss";
2 | 


--------------------------------------------------------------------------------
/examples/tailwind_axum/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/tailwind_axum/public/favicon.ico


--------------------------------------------------------------------------------
/examples/tailwind_axum/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/tailwind_axum/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod app;
 2 | 
 3 | #[cfg(feature = "hydrate")]
 4 | #[wasm_bindgen::prelude::wasm_bindgen]
 5 | pub fn hydrate() {
 6 |     use crate::app::App;
 7 |     console_error_panic_hook::set_once();
 8 |     leptos::mount::hydrate_body(App);
 9 | }
10 | 


--------------------------------------------------------------------------------
/examples/tailwind_csr/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | 
 5 | # Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
 6 | # More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
 7 | Cargo.lock
 8 | 
 9 | # These are backup files generated by rustfmt
10 | **/*.rs.bk
11 | 
12 | # Support playwright testing
13 | node_modules/
14 | test-results/
15 | end2end/playwright-report/
16 | playwright/.cache/
17 | pnpm-lock.yaml
18 | 


--------------------------------------------------------------------------------
/examples/tailwind_csr/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "tailwind-csr-trunk"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dependencies]
 7 | leptos = { path = "../../leptos", features = ["csr"] }
 8 | leptos_meta = { path = "../../meta" }
 9 | leptos_router = { path = "../../router" }
10 | gloo-net = { version = "0.6.0", features = ["http"] }
11 | console_error_panic_hook = { version = "0.1.7" }
12 | 


--------------------------------------------------------------------------------
/examples/tailwind_csr/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |   { path = "../cargo-make/main.toml" },
3 |   { path = "../cargo-make/trunk_server.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/examples/tailwind_csr/end2end/package.json:
--------------------------------------------------------------------------------
 1 | {
 2 |   "name": "end2end",
 3 |   "version": "1.0.0",
 4 |   "description": "",
 5 |   "main": "index.js",
 6 |   "scripts": {},
 7 |   "keywords": [],
 8 |   "author": "",
 9 |   "license": "ISC",
10 |   "devDependencies": {
11 |     "@playwright/test": "^1.28.0"
12 |   }
13 | }
14 | 


--------------------------------------------------------------------------------
/examples/tailwind_csr/end2end/tests/example.spec.ts:
--------------------------------------------------------------------------------
1 | import { test, expect } from "@playwright/test";
2 | 
3 | test("homepage has title 'Leptos + Tailwindcss'", async ({ page }) => {
4 |   await page.goto("http://localhost:8080/");
5 | 
6 |   await expect(page).toHaveTitle("Leptos + Tailwindcss");
7 | });
8 | 


--------------------------------------------------------------------------------
/examples/tailwind_csr/index.html:
--------------------------------------------------------------------------------
 1 | <!DOCTYPE html>
 2 | <html lang="en">
 3 |   <head>
 4 |     <meta charset="utf-8" />
 5 |     <link data-trunk rel="rust" data-wasm-opt="z" />
 6 |     <link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico" />
 7 |     <link data-trunk rel="tailwind-css" href="input.css" />
 8 |     <title>Leptos • Counter with Tailwind</title>
 9 |   </head>
10 | 
11 |   <body></body>
12 | </html>
13 | 


--------------------------------------------------------------------------------
/examples/tailwind_csr/input.css:
--------------------------------------------------------------------------------
1 | @import "tailwindcss";


--------------------------------------------------------------------------------
/examples/tailwind_csr/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/tailwind_csr/public/favicon.ico


--------------------------------------------------------------------------------
/examples/tailwind_csr/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/tailwind_csr/src/main.rs:
--------------------------------------------------------------------------------
 1 | mod app;
 2 | 
 3 | use app::*;
 4 | use leptos::{logging, mount};
 5 | 
 6 | pub fn main() {
 7 |     console_error_panic_hook::set_once();
 8 |     logging::log!("csr mode - mounting to body");
 9 |     mount::mount_to_body(App);
10 | }
11 | 


--------------------------------------------------------------------------------
/examples/timer/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "timer"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [profile.release]
 7 | codegen-units = 1
 8 | lto = true
 9 | 
10 | [dependencies]
11 | leptos = { path = "../../leptos", features = ["csr"] }
12 | console_log = "1.0"
13 | log = "0.4.22"
14 | console_error_panic_hook = "0.1.7"
15 | wasm-bindgen = "0.2.93"
16 | 
17 | [dependencies.web-sys]
18 | version = "0.3.70"
19 | features = ["Window"]
20 | 
21 | [dev-dependencies]
22 | wasm-bindgen-test = "0.3.42"
23 | 


--------------------------------------------------------------------------------
/examples/timer/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/trunk_server.toml" },
4 | ]
5 | 


--------------------------------------------------------------------------------
/examples/timer/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Timer Example
 2 | 
 3 | This example creates a simple timer based on `setInterval` in a client side rendered app with Rust and WASM.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/timer/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>


--------------------------------------------------------------------------------
/examples/timer/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/timer/public/favicon.ico


--------------------------------------------------------------------------------
/examples/timer/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/timer/src/main.rs:
--------------------------------------------------------------------------------
1 | use leptos::prelude::*;
2 | use timer::TimerDemo;
3 | 
4 | pub fn main() {
5 |     console_error_panic_hook::set_once();
6 |     mount_to_body(TimerDemo)
7 | }
8 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/.gitignore:
--------------------------------------------------------------------------------
1 | .leptos.kdl


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = [
 2 |   { path = "../cargo-make/main.toml" },
 3 |   { path = "../cargo-make/cargo-leptos-webdriver-test.toml" },
 4 | ]
 5 | 
 6 | [env]
 7 | CLIENT_PROCESS_NAME = "todo_app_sqlite"
 8 | 
 9 | [tasks.test-ui]
10 | cwd = "./e2e"
11 | command = "cargo"
12 | args = ["make", "test-ui", "${@}"]
13 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/Todos.db:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/todo_app_sqlite/Todos.db


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/e2e/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "todo_app_sqlite_e2e"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dev-dependencies]
 7 | anyhow = "1.0"
 8 | async-trait = "0.1.81"
 9 | cucumber = "0.21.1"
10 | fantoccini = "0.21.1"
11 | pretty_assertions = "1.4"
12 | serde_json = "1.0"
13 | tokio = { version = "1.39", features = ["macros", "rt-multi-thread", "time"] }
14 | url = "2.5"
15 | 
16 | [[test]]
17 | name = "app_suite"
18 | harness = false    # Allow Cucumber to print output instead of libtest
19 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/e2e/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = { path = "../../cargo-make/main.toml" }
 2 | 
 3 | [tasks.test]
 4 | env = { RUN_AUTOMATICALLY = false }
 5 | condition = { env_true = ["RUN_AUTOMATICALLY"] }
 6 | 
 7 | [tasks.ci]
 8 | 
 9 | [tasks.test-ui]
10 | command = "cargo"
11 | args = [
12 |   "test",
13 |   "--test",
14 |   "app_suite",
15 |   "--",
16 |   "--retry",
17 |   "2",
18 |   "--fail-fast",
19 |   "${@}",
20 | ]
21 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/e2e/features/add_todo.feature:
--------------------------------------------------------------------------------
 1 | @add_todo
 2 | Feature: Add Todo
 3 | 
 4 |     Background:
 5 |         Given I see the app
 6 | 
 7 |     @add_todo-see
 8 |     Scenario: Should see the todo
 9 |         Given I set the todo as Buy Bread
10 |         When I click the Add button
11 |         Then I see the todo named Buy Bread
12 | 
13 |     # @allow.skipped
14 |     @add_todo-style
15 |     Scenario: Should see the pending todo
16 |         When I add a todo as Buy Oranges
17 |         Then I see the pending todo
18 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/e2e/features/open_app.feature:
--------------------------------------------------------------------------------
 1 | @open_app
 2 | Feature: Open App
 3 | 
 4 |   @open_app-title
 5 |   Scenario: Should see the home page title
 6 |     When I open the app
 7 |     Then I see the page title is My Tasks
 8 | 
 9 |   @open_app-label
10 |   Scenario: Should see the input label
11 |     When I open the app
12 |     Then I see the label of the input is Add a Todo


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/e2e/tests/app_suite.rs:
--------------------------------------------------------------------------------
 1 | mod fixtures;
 2 | 
 3 | use anyhow::Result;
 4 | use cucumber::World;
 5 | use fixtures::world::AppWorld;
 6 | 
 7 | #[tokio::main]
 8 | async fn main() -> Result<()> {
 9 |     AppWorld::cucumber()
10 |         .fail_on_skipped()
11 |         .run_and_exit("./features")
12 |         .await;
13 |     Ok(())
14 | }
15 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/e2e/tests/fixtures/mod.rs:
--------------------------------------------------------------------------------
1 | pub mod action;
2 | pub mod check;
3 | pub mod find;
4 | pub mod world;
5 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/migrations/20221118172000_create_todo_table.sql:
--------------------------------------------------------------------------------
1 | CREATE TABLE IF NOT EXISTS todos
2 | (
3 |   id          INTEGER NOT NULL PRIMARY KEY,
4 |   title       VARCHAR,
5 |   completed   BOOLEAN
6 | );


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/todo_app_sqlite/public/favicon.ico


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod todo;
 2 | 
 3 | #[cfg(feature = "hydrate")]
 4 | #[wasm_bindgen::prelude::wasm_bindgen]
 5 | pub fn hydrate() {
 6 |     use crate::todo::*;
 7 |     console_error_panic_hook::set_once();
 8 |     _ = console_log::init_with_level(log::Level::Debug);
 9 | 
10 |     leptos::mount::hydrate_body(TodoApp);
11 | }
12 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite/style.css:
--------------------------------------------------------------------------------
1 | .pending {
2 | 	color: purple;
3 | }


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = [
 2 |     { path = "../cargo-make/main.toml" },
 3 |     { path = "../cargo-make/cargo-leptos-webdriver-test.toml" },
 4 | ]
 5 | 
 6 | [env]
 7 | CLIENT_PROCESS_NAME = "todo_app_sqlite_axum"
 8 | 
 9 | [tasks.test-ui]
10 | cwd = "./e2e"
11 | command = "cargo"
12 | args = ["make", "test-ui", "${@}"]
13 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/Todos.db:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/todo_app_sqlite_axum/Todos.db


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/e2e/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "todo_app_sqlite_axum_e2e"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dev-dependencies]
 7 | anyhow = "1.0"
 8 | async-trait = "0.1.81"
 9 | cucumber = "0.21.1"
10 | fantoccini = "0.21.1"
11 | pretty_assertions = "1.4"
12 | serde_json = "1.0"
13 | tokio = { version = "1.39", features = ["macros", "rt-multi-thread", "time"] }
14 | url = "2.5"
15 | 
16 | [[test]]
17 | name = "app_suite"
18 | harness = false    # Allow Cucumber to print output instead of libtest
19 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/e2e/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = { path = "../../cargo-make/main.toml" }
 2 | 
 3 | [tasks.test]
 4 | env = { RUN_AUTOMATICALLY = false }
 5 | condition = { env_true = ["RUN_AUTOMATICALLY"] }
 6 | 
 7 | [tasks.ci]
 8 | 
 9 | [tasks.test-ui]
10 | command = "cargo"
11 | args = [
12 |   "test",
13 |   "--test",
14 |   "app_suite",
15 |   "--",
16 |   "--retry",
17 |   "2",
18 |   "--fail-fast",
19 |   "${@}",
20 | ]
21 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/e2e/features/add_todo.feature:
--------------------------------------------------------------------------------
 1 | @add_todo
 2 | Feature: Add Todo
 3 | 
 4 |     Background:
 5 |         Given I see the app
 6 | 
 7 |     @add_todo-see
 8 |     Scenario: Should see the todo
 9 |         Given I set the todo as Buy Bread
10 |         When I click the Add button
11 |         Then I see the todo named Buy Bread
12 | 
13 |     @add_todo-style
14 |     Scenario: Should see the pending todo
15 |         When I add a todo as Buy Oranges
16 |         Then I see the pending todo
17 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/e2e/features/open_app.feature:
--------------------------------------------------------------------------------
 1 | @open_app
 2 | Feature: Open App
 3 | 
 4 |   @open_app-title
 5 |   Scenario: Should see the home page title
 6 |     When I open the app
 7 |     Then I see the page title is My Tasks
 8 | 
 9 |   @open_app-label
10 |   Scenario: Should see the input label
11 |     When I open the app
12 |     Then I see the label of the input is Add a Todo


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/e2e/tests/app_suite.rs:
--------------------------------------------------------------------------------
 1 | mod fixtures;
 2 | 
 3 | use anyhow::Result;
 4 | use cucumber::World;
 5 | use fixtures::world::AppWorld;
 6 | 
 7 | #[tokio::main]
 8 | async fn main() -> Result<()> {
 9 |     AppWorld::cucumber()
10 |         .fail_on_skipped()
11 |         .run_and_exit("./features")
12 |         .await;
13 |     Ok(())
14 | }
15 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/e2e/tests/fixtures/mod.rs:
--------------------------------------------------------------------------------
1 | pub mod action;
2 | pub mod check;
3 | pub mod find;
4 | pub mod world;
5 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/migrations/20221118172000_create_todo_table.sql:
--------------------------------------------------------------------------------
1 | 
2 | CREATE TABLE IF NOT EXISTS todos
3 | (
4 |   id          INTEGER NOT NULL PRIMARY KEY,
5 |   title       VARCHAR,
6 |   completed   BOOLEAN
7 | );


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/todo_app_sqlite_axum/public/favicon.ico


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod error_template;
 2 | pub mod errors;
 3 | pub mod todo;
 4 | 
 5 | #[cfg(feature = "hydrate")]
 6 | #[wasm_bindgen::prelude::wasm_bindgen]
 7 | pub fn hydrate() {
 8 |     use crate::todo::TodoApp;
 9 |     console_error_panic_hook::set_once();
10 |     leptos::mount::hydrate_body(TodoApp);
11 | }
12 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_axum/style.css:
--------------------------------------------------------------------------------
1 | .pending {
2 | 	color: purple;
3 | }
4 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = [
 2 |     { path = "../cargo-make/main.toml" },
 3 |     { path = "../cargo-make/cargo-leptos-webdriver-test.toml" },
 4 | ]
 5 | 
 6 | [env]
 7 | CLIENT_PROCESS_NAME = "todo_app_sqlite_csr"
 8 | 
 9 | [tasks.test-ui]
10 | cwd = "./e2e"
11 | command = "cargo"
12 | args = ["make", "test-ui", "${@}"]
13 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/Todos.db:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/todo_app_sqlite_csr/Todos.db


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/e2e/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "todo_app_sqlite_csr_e2e"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dev-dependencies]
 7 | anyhow = "1.0"
 8 | async-trait = "0.1.81"
 9 | cucumber = "0.21.1"
10 | fantoccini = "0.21.1"
11 | pretty_assertions = "1.4"
12 | serde_json = "1.0"
13 | tokio = { version = "1.39", features = ["macros", "rt-multi-thread", "time"] }
14 | url = "2.5"
15 | 
16 | [[test]]
17 | name = "app_suite"
18 | harness = false    # Allow Cucumber to print output instead of libtest
19 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/e2e/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = { path = "../../cargo-make/main.toml" }
 2 | 
 3 | [tasks.test]
 4 | env = { RUN_AUTOMATICALLY = false }
 5 | condition = { env_true = ["RUN_AUTOMATICALLY"] }
 6 | 
 7 | [tasks.ci]
 8 | 
 9 | [tasks.test-ui]
10 | command = "cargo"
11 | args = [
12 |   "test",
13 |   "--test",
14 |   "app_suite",
15 |   "--",
16 |   "--retry",
17 |   "5",
18 |   "--fail-fast",
19 |   "${@}",
20 | ]
21 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/e2e/features/add_todo.feature:
--------------------------------------------------------------------------------
 1 | @add_todo
 2 | Feature: Add Todo
 3 | 
 4 |     Background:
 5 |         Given I see the app
 6 | 
 7 |     @add_todo-see
 8 |     Scenario: Should see the todo
 9 |         Given I set the todo as Buy Bread
10 |         When I click the Add button
11 |         Then I see the todo named Buy Bread
12 | 
13 |     @add_todo-style
14 |     Scenario: Should see the pending todo
15 |         When I add a todo as Buy Oranges
16 |         Then I see the pending todo
17 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/e2e/features/open_app.feature:
--------------------------------------------------------------------------------
 1 | @open_app
 2 | Feature: Open App
 3 | 
 4 |   @open_app-title
 5 |   Scenario: Should see the home page title
 6 |     When I open the app
 7 |     Then I see the page title is My Tasks
 8 | 
 9 |   @open_app-label
10 |   Scenario: Should see the input label
11 |     When I open the app
12 |     Then I see the label of the input is Add a Todo


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/e2e/tests/app_suite.rs:
--------------------------------------------------------------------------------
 1 | mod fixtures;
 2 | 
 3 | use anyhow::Result;
 4 | use cucumber::World;
 5 | use fixtures::world::AppWorld;
 6 | 
 7 | #[tokio::main]
 8 | async fn main() -> Result<()> {
 9 |     AppWorld::cucumber()
10 |         .fail_on_skipped()
11 |         .run_and_exit("./features")
12 |         .await;
13 |     Ok(())
14 | }
15 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/e2e/tests/fixtures/mod.rs:
--------------------------------------------------------------------------------
1 | pub mod action;
2 | pub mod check;
3 | pub mod find;
4 | pub mod world;
5 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/migrations/20221118172000_create_todo_table.sql:
--------------------------------------------------------------------------------
1 | 
2 | CREATE TABLE IF NOT EXISTS todos
3 | (
4 |   id          INTEGER NOT NULL PRIMARY KEY,
5 |   title       VARCHAR,
6 |   completed   BOOLEAN
7 | );


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/todo_app_sqlite_csr/public/favicon.ico


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod error_template;
 2 | pub mod errors;
 3 | #[cfg(feature = "ssr")]
 4 | pub mod fallback;
 5 | pub mod todo;
 6 | 
 7 | #[cfg_attr(feature = "csr", wasm_bindgen::prelude::wasm_bindgen)]
 8 | pub fn hydrate() {
 9 |     use crate::todo::*;
10 |     console_error_panic_hook::set_once();
11 |     leptos::mount::mount_to_body(TodoApp);
12 | }
13 | 


--------------------------------------------------------------------------------
/examples/todo_app_sqlite_csr/style.css:
--------------------------------------------------------------------------------
1 | .pending {
2 | 	color: purple;
3 | }
4 | 


--------------------------------------------------------------------------------
/examples/todomvc/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/trunk_server.toml" },
4 | ]
5 | 
6 | [tasks.setup-node]
7 | env = { SETUP_NODE = false }
8 | condition = { env_true = ["SETUP_NODE"] }
9 | 


--------------------------------------------------------------------------------
/examples/todomvc/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos TodoMVC
 2 | 
 3 | This is a Leptos implementation of the TodoMVC example common to many frameworks. This is a relatively-simple application but shows off features like interaction between components and state management.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `trunk serve --open` to run this example.
12 | 


--------------------------------------------------------------------------------
/examples/todomvc/index.html:
--------------------------------------------------------------------------------
 1 | <!DOCTYPE html>
 2 | <html lang="en">
 3 | 	<head>
 4 | 		<meta charset="utf-8">
 5 | 		<meta name="viewport" content="width=device-width, initial-scale=1">
 6 | 		<link data-trunk rel="css" href="./node_modules/todomvc-common/base.css">
 7 | 		<link data-trunk rel="css" href="./node_modules/todomvc-app-css/index.css">
 8 | 		<title>Leptos • TodoMVC</title>
 9 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
10 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
11 | 	</head>
12 | 	<body></body>
13 | </html>


--------------------------------------------------------------------------------
/examples/todomvc/node_modules/todomvc-app-css/readme.md:
--------------------------------------------------------------------------------
 1 | # todomvc-app-css
 2 | 
 3 | > CSS for TodoMVC apps
 4 | 
 5 | ![](screenshot.png)
 6 | 
 7 | 
 8 | ## Install
 9 | 
10 | 
11 | ```
12 | $ npm install todomvc-app-css
13 | ```
14 | 
15 | 
16 | ## Getting started
17 | 
18 | ```html
19 | <link rel="stylesheet" href="node_modules/todomvc-app-css/index.css">
20 | ```
21 | 
22 | See the [TodoMVC app template](https://github.com/tastejs/todomvc-app-template).
23 | 
24 | 
25 | ## License
26 | 
27 | CC-BY-4.0 © [Sindre Sorhus](https://sindresorhus.com)
28 | 


--------------------------------------------------------------------------------
/examples/todomvc/node_modules/todomvc-common/package.json:
--------------------------------------------------------------------------------
 1 | {
 2 | 	"name": "todomvc-common",
 3 | 	"version": "1.0.5",
 4 | 	"description": "Common TodoMVC utilities used by our apps",
 5 | 	"license": "MIT",
 6 | 	"repository": "tastejs/todomvc-common",
 7 | 	"author": "TasteJS",
 8 | 	"main": "base.js",
 9 | 	"style": "base.css",
10 | 	"files": [
11 | 		"base.js",
12 | 		"base.css"
13 | 	],
14 | 	"keywords": [
15 | 		"todomvc",
16 | 		"tastejs",
17 | 		"util",
18 | 		"utilities"
19 | 	]
20 | }
21 | 


--------------------------------------------------------------------------------
/examples/todomvc/node_modules/todomvc-common/readme.md:
--------------------------------------------------------------------------------
 1 | # todomvc-common
 2 | 
 3 | > Common TodoMVC utilities used by our apps
 4 | 
 5 | 
 6 | ## Install
 7 | 
 8 | ```
 9 | $ npm install todomvc-common
10 | ```
11 | 
12 | 
13 | ## License
14 | 
15 | MIT © [TasteJS](http://tastejs.com)
16 | 


--------------------------------------------------------------------------------
/examples/todomvc/package.json:
--------------------------------------------------------------------------------
 1 | {
 2 |   "name": "todomvc",
 3 |   "version": "0.1.0",
 4 |   "description": "TodoMVC implemented in the Leptos framework for Rust",
 5 |   "author": "Greg Johnston <greg.johnston@gmail.com>",
 6 |   "license": "ISC",
 7 |   "dependencies": {
 8 |     "todomvc-app-css": "^2.4.2",
 9 |     "todomvc-common": "^1.0.5"
10 |   }
11 | }
12 | 


--------------------------------------------------------------------------------
/examples/todomvc/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/todomvc/public/favicon.ico


--------------------------------------------------------------------------------
/examples/todomvc/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | targets = ["wasm32-unknown-unknown"]
3 | 


--------------------------------------------------------------------------------
/examples/todomvc/src/main.rs:
--------------------------------------------------------------------------------
1 | pub use todomvc::*;
2 | 
3 | fn main() {
4 |     console_error_panic_hook::set_once();
5 |     leptos::mount::mount_to_body(TodoMVC)
6 | }
7 | 


--------------------------------------------------------------------------------
/examples/websocket/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = [
 2 |   { path = "../cargo-make/main.toml" },
 3 |   { path = "../cargo-make/cargo-leptos-webdriver-test.toml" },
 4 | ]
 5 | 
 6 | [env]
 7 | CLIENT_PROCESS_NAME = "websocket"
 8 | 
 9 | [tasks.test-ui]
10 | cwd = "./e2e"
11 | command = "cargo"
12 | args = ["make", "test-ui", "${@}"]
13 | 


--------------------------------------------------------------------------------
/examples/websocket/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos WebSocket
 2 | 
 3 | This example creates a basic WebSocket echo app.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## E2E Testing
10 | 
11 | See the [E2E README](./e2e/README.md) for more information about the testing strategy.
12 | 
13 | ## Rendering
14 | 
15 | See the [SSR Notes](../SSR_NOTES.md) for more information about Server Side Rendering.
16 | 
17 | ## Quick Start
18 | 
19 | Run `cargo leptos watch` to run this example.
20 | 


--------------------------------------------------------------------------------
/examples/websocket/e2e/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "websocket_e2e"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dev-dependencies]
 7 | anyhow = "1.0"
 8 | async-trait = "0.1.81"
 9 | cucumber = "0.21.1"
10 | fantoccini = "0.21.1"
11 | pretty_assertions = "1.4"
12 | serde_json = "1.0"
13 | tokio = { version = "1.39", features = ["macros", "rt-multi-thread", "time"] }
14 | url = "2.5"
15 | 
16 | [[test]]
17 | name = "app_suite"
18 | harness = false    # Allow Cucumber to print output instead of libtest
19 | 


--------------------------------------------------------------------------------
/examples/websocket/e2e/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = { path = "../../cargo-make/main.toml" }
 2 | 
 3 | [tasks.test]
 4 | env = { RUN_AUTOMATICALLY = false }
 5 | condition = { env_true = ["RUN_AUTOMATICALLY"] }
 6 | 
 7 | [tasks.ci]
 8 | 
 9 | [tasks.test-ui]
10 | command = "cargo"
11 | args = [
12 |   "test",
13 |   "--test",
14 |   "app_suite",
15 |   "--",
16 |   "--retry",
17 |   "2",
18 |   "--fail-fast",
19 |   "${@}",
20 | ]
21 | 


--------------------------------------------------------------------------------
/examples/websocket/e2e/features/echo_client_error.feature:
--------------------------------------------------------------------------------
 1 | @echo_client_error
 2 | Feature: Echo Client Error
 3 | 
 4 |     Background:
 5 |         Given I see the app
 6 | 
 7 |     @echo_client_error-see-fifth-input-error
 8 |     Scenario: Should see the client error
 9 |         Given I add a text as abcde
10 |         Then I see the label of the input is Error(ServerFnErrorWrapper(Registration("Error generated from client")))
11 | 


--------------------------------------------------------------------------------
/examples/websocket/e2e/features/echo_server_error.feature:
--------------------------------------------------------------------------------
 1 | @echo_server_error
 2 | Feature: Echo Server Error
 3 | 
 4 |     Background:
 5 |         Given I see the app
 6 | 
 7 |     @echo_server_error-see-third-input-error
 8 |     Scenario: Should see the server error
 9 |         Given I add a text as abc
10 |         Then I see the label of the input is Error(ServerFnErrorWrapper(Registration("Error generated from server")))
11 | 


--------------------------------------------------------------------------------
/examples/websocket/e2e/features/echo_text.feature:
--------------------------------------------------------------------------------
 1 | @echo_text
 2 | Feature: Echo Text
 3 | 
 4 |     Background:
 5 |         Given I see the app
 6 | 
 7 |     @echo_text-see-first-input
 8 |     Scenario: Should see the label
 9 |         Given I add a text as a
10 |         Then I see the label of the input is A
11 | 
12 |     @add_text-see-second-input
13 |     Scenario: Should see the label
14 |         Given I add a text as ab
15 |         Then I see the label of the input is AB
16 | 
17 | 
18 | 


--------------------------------------------------------------------------------
/examples/websocket/e2e/features/open_app.feature:
--------------------------------------------------------------------------------
1 | @open_app
2 | Feature: Open App
3 | 
4 |   @open_app-title
5 |   Scenario: Should see the home page title
6 |     When I open the app
7 |     Then I see the page title is Simple Echo WebSocket Communication
8 | 


--------------------------------------------------------------------------------
/examples/websocket/e2e/tests/app_suite.rs:
--------------------------------------------------------------------------------
 1 | mod fixtures;
 2 | 
 3 | use anyhow::Result;
 4 | use cucumber::World;
 5 | use fixtures::world::AppWorld;
 6 | 
 7 | #[tokio::main]
 8 | async fn main() -> Result<()> {
 9 |     AppWorld::cucumber()
10 |         .fail_on_skipped()
11 |         .run_and_exit("./features")
12 |         .await;
13 |     Ok(())
14 | }
15 | 


--------------------------------------------------------------------------------
/examples/websocket/e2e/tests/fixtures/action.rs:
--------------------------------------------------------------------------------
 1 | use super::{find, world::HOST};
 2 | use anyhow::Result;
 3 | use fantoccini::Client;
 4 | use std::result::Result::Ok;
 5 | 
 6 | pub async fn goto_path(client: &Client, path: &str) -> Result<()> {
 7 |     let url = format!("{}{}", HOST, path);
 8 |     client.goto(&url).await?;
 9 | 
10 |     Ok(())
11 | }
12 | 
13 | pub async fn fill_input(client: &Client, text: &str) -> Result<()> {
14 |     let textbox = find::input(client).await;
15 |     textbox.send_keys(text).await?;
16 | 
17 |     Ok(())
18 | }
19 | 


--------------------------------------------------------------------------------
/examples/websocket/e2e/tests/fixtures/find.rs:
--------------------------------------------------------------------------------
 1 | use fantoccini::{elements::Element, Client, Locator};
 2 | 
 3 | pub async fn input(client: &Client) -> Element {
 4 |     let textbox = client
 5 |         .wait()
 6 |         .for_element(Locator::Css("input"))
 7 |         .await
 8 |         .expect("websocket textbox not found");
 9 | 
10 |     textbox
11 | }
12 | 


--------------------------------------------------------------------------------
/examples/websocket/e2e/tests/fixtures/mod.rs:
--------------------------------------------------------------------------------
1 | pub mod action;
2 | pub mod check;
3 | pub mod find;
4 | pub mod world;
5 | 


--------------------------------------------------------------------------------
/examples/websocket/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/websocket/public/favicon.ico


--------------------------------------------------------------------------------
/examples/websocket/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod websocket;
 2 | 
 3 | #[cfg(feature = "hydrate")]
 4 | #[wasm_bindgen::prelude::wasm_bindgen]
 5 | pub fn hydrate() {
 6 |     use crate::websocket::App;
 7 |     console_error_panic_hook::set_once();
 8 |     leptos::mount::hydrate_body(App);
 9 | }
10 | 


--------------------------------------------------------------------------------
/examples/websocket/style.css:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/examples/websocket/style.css


--------------------------------------------------------------------------------
/hydration_context/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/integrations/actix/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../../cargo-make/main.toml" }
2 | 
3 | [tasks.check-format]
4 | env = { LEPTOS_PROJECT_DIRECTORY = "../../" }
5 | 


--------------------------------------------------------------------------------
/integrations/axum/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../../cargo-make/main.toml" }
2 | 
3 | [tasks.check-format]
4 | env = { LEPTOS_PROJECT_DIRECTORY = "../../" }
5 | 


--------------------------------------------------------------------------------
/integrations/utils/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../../cargo-make/main.toml" }
2 | 
3 | [tasks.check-format]
4 | env = { LEPTOS_PROJECT_DIRECTORY = "../../" }
5 | 


--------------------------------------------------------------------------------
/leptos/build.rs:
--------------------------------------------------------------------------------
 1 | use rustc_version::{version_meta, Channel};
 2 | 
 3 | fn main() {
 4 |     let target = std::env::var("TARGET").unwrap_or_default();
 5 | 
 6 |     // Set cfg flags depending on release channel
 7 |     if matches!(version_meta().unwrap().channel, Channel::Nightly) {
 8 |         println!("cargo:rustc-cfg=rustc_nightly");
 9 |     }
10 |     // Set cfg flag for getrandom wasm_js
11 |     if target == "wasm32-unknown-unknown" {
12 |         // Set a custom cfg flag for wasm builds
13 |         println!("cargo:rustc-cfg=getrandom_backend=\"wasm_js\"");
14 |     }
15 | }
16 | 


--------------------------------------------------------------------------------
/leptos/src/from_form_data.rs:
--------------------------------------------------------------------------------
1 | 
2 | 


--------------------------------------------------------------------------------
/leptos/src/hydration/hydration_script.js:
--------------------------------------------------------------------------------
1 | (function (root, pkg_path, output_name, wasm_output_name) {
2 | 	import(`${root}/${pkg_path}/${output_name}.js`)
3 | 		.then(mod => {
4 | 			mod.default({module_or_path: `${root}/${pkg_path}/${wasm_output_name}.wasm`}).then(() => {
5 | 				mod.hydrate();
6 | 			});
7 | 		})
8 | })
9 | 


--------------------------------------------------------------------------------
/leptos_config/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/leptos_dom/.cargo/config.toml:
--------------------------------------------------------------------------------
1 | # [build]
2 | # target = "wasm32-unknown-unknown"


--------------------------------------------------------------------------------
/leptos_dom/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/leptos_dom/examples/test-bench/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "test-bench"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dependencies]
 7 | console_error_panic_hook = "0.1.7"
 8 | gloo = { version = "0.11.0", features = ["futures"] }
 9 | leptos = { path = "../../../leptos", features = ["nightly", "csr", "tracing"] }
10 | tracing = "0.1.40"
11 | tracing-subscriber = "0.3.18"
12 | tracing-subscriber-wasm = "0.1.0"
13 | 
14 | [workspace]
15 | 


--------------------------------------------------------------------------------
/leptos_dom/examples/test-bench/index.html:
--------------------------------------------------------------------------------
 1 | <!DOCTYPE html>
 2 | <html lang="en">
 3 |   <head>
 4 |     <meta charset="UTF-8" />
 5 |     <meta http-equiv="X-UA-Compatible" content="IE=edge" />
 6 |     <meta name="viewport" content="width=device-width, initial-scale=1.0" />
 7 |     <title>Leptos DOM v2 Test Bench</title>
 8 |   </head>
 9 |   <body></body>
10 | </html>
11 | 


--------------------------------------------------------------------------------
/leptos_dom/examples/view-tests/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "view-tests"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | [dependencies]
 7 | leptos = { path = "../../leptos", features = ["csr", "nightly"] }
 8 | console_log = "1.0"
 9 | log = "0.4.0"
10 | console_error_panic_hook = "0.1.7"
11 | 
12 | [dev-dependencies]
13 | wasm-bindgen-test = "0.3.0"
14 | 


--------------------------------------------------------------------------------
/leptos_dom/examples/view-tests/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
5 | 	</head>
6 | 	<body></body>
7 | </html>


--------------------------------------------------------------------------------
/leptos_dom/src/lib.rs:
--------------------------------------------------------------------------------
 1 | #![deny(missing_docs)]
 2 | #![forbid(unsafe_code)]
 3 | 
 4 | //! DOM helpers for Leptos.
 5 | 
 6 | pub mod helpers;
 7 | #[doc(hidden)]
 8 | pub mod macro_helpers;
 9 | 
10 | /// Utilities for simple isomorphic logging to the console or terminal.
11 | pub mod logging;
12 | 


--------------------------------------------------------------------------------
/leptos_dom/src/macro_helpers/mod.rs:
--------------------------------------------------------------------------------
1 | #[cfg(feature = "trace-component-props")]
2 | #[doc(hidden)]
3 | pub mod tracing_property;
4 | 


--------------------------------------------------------------------------------
/leptos_hot_reload/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/leptos_macro/.gitignore:
--------------------------------------------------------------------------------
1 | target
2 | Cargo.lock


--------------------------------------------------------------------------------
/leptos_macro/build.rs:
--------------------------------------------------------------------------------
1 | use rustc_version::{version_meta, Channel};
2 | 
3 | fn main() {
4 |     // Set cfg flags depending on release channel
5 |     if matches!(version_meta().unwrap().channel, Channel::Nightly) {
6 |         println!("cargo:rustc-cfg=rustc_nightly");
7 |     }
8 | }
9 | 


--------------------------------------------------------------------------------
/leptos_macro/example/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "example"
 3 | version = "0.1.0"
 4 | edition = "2021"
 5 | 
 6 | # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
 7 | 
 8 | [dependencies]
 9 | leptos.path = "../../leptos"
10 | 
11 | [workspace]
12 | 


--------------------------------------------------------------------------------
/leptos_macro/tests/ui.rs:
--------------------------------------------------------------------------------
 1 | #[cfg(not(feature = "__internal_erase_components"))]
 2 | #[test]
 3 | fn ui() {
 4 |     let t = trybuild::TestCases::new();
 5 |     #[cfg(all(feature = "nightly", rustc_nightly))]
 6 |     t.compile_fail("tests/ui/component.rs");
 7 |     #[cfg(all(feature = "nightly", rustc_nightly))]
 8 |     t.compile_fail("tests/ui/component_absolute.rs");
 9 |     t.compile_fail("tests/ui/server.rs");
10 | }
11 | 


--------------------------------------------------------------------------------
/leptos_server/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/meta/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/next_tuple/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "next_tuple"
 3 | version = "0.1.0"
 4 | authors = ["Greg Johnston"]
 5 | license = "MIT"
 6 | readme = "../README.md"
 7 | repository = "https://github.com/leptos-rs/leptos"
 8 | description = "A trait to build and extend tuples."
 9 | rust-version.workspace = true
10 | edition.workspace = true
11 | 
12 | [dependencies]
13 | 


--------------------------------------------------------------------------------
/next_tuple/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/next_tuple/README.md:
--------------------------------------------------------------------------------
1 | Allows extending a tuple, or creating a new tuple, by adding the next value.
2 | 


--------------------------------------------------------------------------------
/oco/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/or_poisoned/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [package]
 2 | name = "or_poisoned"
 3 | version = "0.1.0"
 4 | authors = ["Greg Johnston"]
 5 | license = "MIT"
 6 | readme = "../README.md"
 7 | repository = "https://github.com/leptos-rs/leptos"
 8 | description = "Unwrap std lock guards in a semantic way."
 9 | rust-version.workspace = true
10 | edition.workspace = true
11 | 
12 | [dependencies]
13 | 


--------------------------------------------------------------------------------
/or_poisoned/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/projects/bevy3d_ui/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z"/>
5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>


--------------------------------------------------------------------------------
/projects/bevy3d_ui/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/bevy3d_ui/public/favicon.ico


--------------------------------------------------------------------------------
/projects/bevy3d_ui/src/demos/bevydemo1/eventqueue/mod.rs:
--------------------------------------------------------------------------------
1 | pub mod events;
2 | pub mod plugin;
3 | 


--------------------------------------------------------------------------------
/projects/bevy3d_ui/src/demos/bevydemo1/mod.rs:
--------------------------------------------------------------------------------
1 | pub mod eventqueue;
2 | pub mod scene;
3 | pub mod state;
4 | 


--------------------------------------------------------------------------------
/projects/bevy3d_ui/src/demos/mod.rs:
--------------------------------------------------------------------------------
1 | pub mod bevydemo1;
2 | 


--------------------------------------------------------------------------------
/projects/bevy3d_ui/src/main.rs:
--------------------------------------------------------------------------------
 1 | mod demos;
 2 | mod routes;
 3 | use leptos::prelude::*;
 4 | use routes::RootPage;
 5 | 
 6 | pub fn main() {
 7 |     // Bevy will output a lot of debug info to the console when this is enabled.
 8 |     //_ = console_log::init_with_level(log::Level::Debug);
 9 |     console_error_panic_hook::set_once();
10 |     mount_to_body(|| view! { <RootPage/> })
11 | }
12 | 


--------------------------------------------------------------------------------
/projects/counter_dwarf_debug/.gitignore:
--------------------------------------------------------------------------------
1 | # For this example we want to include the vscode files
2 | !.vscode
3 | 


--------------------------------------------------------------------------------
/projects/counter_dwarf_debug/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [workspace]
 2 | # The empty workspace here is to keep rust-analyzer satisfied
 3 | 
 4 | [package]
 5 | name = "counter_dwarf_debug"
 6 | version = "0.1.0"
 7 | edition = "2021"
 8 | 
 9 | [profile.release]
10 | codegen-units = 1
11 | lto = true
12 | 
13 | [dependencies]
14 | leptos = { path = "../../leptos", features = ["csr"] }
15 | console_log = "1.0"
16 | log = "0.4.22"
17 | console_error_panic_hook = "0.1.7"
18 | 
19 | [dev-dependencies]
20 | wasm-bindgen = "0.2.92"
21 | wasm-bindgen-test = "0.3.42"
22 | web-sys = "0.3.69"
23 | 


--------------------------------------------------------------------------------
/projects/counter_dwarf_debug/Trunk.toml:
--------------------------------------------------------------------------------
1 | [serve]
2 | address = "127.0.0.1"
3 | port = 4001
4 | open = false
5 | 


--------------------------------------------------------------------------------
/projects/counter_dwarf_debug/img/breakpoint1.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/counter_dwarf_debug/img/breakpoint1.png


--------------------------------------------------------------------------------
/projects/counter_dwarf_debug/img/breakpoint2.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/counter_dwarf_debug/img/breakpoint2.png


--------------------------------------------------------------------------------
/projects/counter_dwarf_debug/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-keep-debug="true" data-wasm-opt="z"/>
5 | 		<link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico"/>
6 | 	</head>
7 | 	<body></body>
8 | </html>
9 | 


--------------------------------------------------------------------------------
/projects/counter_dwarf_debug/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/counter_dwarf_debug/public/favicon.ico


--------------------------------------------------------------------------------
/projects/counter_dwarf_debug/rust-toolchain.toml:
--------------------------------------------------------------------------------
1 | [toolchain]
2 | channel = "stable" # test change
3 | 


--------------------------------------------------------------------------------
/projects/counter_dwarf_debug/src/main.rs:
--------------------------------------------------------------------------------
 1 | use counter_dwarf_debug::SimpleCounter;
 2 | use leptos::*;
 3 | 
 4 | pub fn main() {
 5 |     _ = console_log::init_with_level(log::Level::Debug);
 6 |     console_error_panic_hook::set_once();
 7 |     mount_to_body(|| {
 8 |         view! {
 9 |             <SimpleCounter
10 |                 initial_value=0
11 |                 step=1
12 |             />
13 |         }
14 |     })
15 | }
16 | 


--------------------------------------------------------------------------------
/projects/hexagonal-architecture/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | debug/
 4 | target/
 5 | 
 6 | # Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
 7 | # More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
 8 | Cargo.lock
 9 | 
10 | # These are backup files generated by rustfmt
11 | **/*.rs.bk
12 | 
13 | # MSVC Windows builds of rustc generate these, which store debugging information
14 | *.pdb
15 | 


--------------------------------------------------------------------------------
/projects/hexagonal-architecture/end2end/package.json:
--------------------------------------------------------------------------------
 1 | {
 2 |   "name": "end2end",
 3 |   "version": "1.0.0",
 4 |   "description": "",
 5 |   "main": "index.js",
 6 |   "scripts": {},
 7 |   "keywords": [],
 8 |   "author": "",
 9 |   "license": "ISC",
10 |   "devDependencies": {
11 |     "@playwright/test": "^1.44.1",
12 |     "@types/node": "^20.12.12",
13 |     "typescript": "^5.4.5"
14 |   }
15 | }
16 | 


--------------------------------------------------------------------------------
/projects/hexagonal-architecture/end2end/tests/example.spec.ts:
--------------------------------------------------------------------------------
 1 | import { test, expect } from "@playwright/test";
 2 | 
 3 | test("homepage has title and heading text", async ({ page }) => {
 4 |   await page.goto("http://localhost:3000/");
 5 | 
 6 |   await expect(page).toHaveTitle("Welcome to Leptos");
 7 | 
 8 |   await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
 9 | });
10 | 


--------------------------------------------------------------------------------
/projects/hexagonal-architecture/leptos_hexagonal_architecture.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/hexagonal-architecture/leptos_hexagonal_architecture.png


--------------------------------------------------------------------------------
/projects/hexagonal-architecture/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/hexagonal-architecture/public/favicon.ico


--------------------------------------------------------------------------------
/projects/hexagonal-architecture/src/ui_types.rs:
--------------------------------------------------------------------------------
1 | use serde::{Deserialize, Serialize};
2 | 
3 | #[derive(Serialize, Deserialize)]
4 | pub struct UiMappingFromDomainData;
5 | #[derive(Serialize, Deserialize)]
6 | pub struct UiMappingFromDomainData2;
7 | #[derive(Serialize, Deserialize)]
8 | pub struct UiMappingFromDomainData3;
9 | 


--------------------------------------------------------------------------------
/projects/hexagonal-architecture/style/main.scss:
--------------------------------------------------------------------------------
1 | body {
2 | 	font-family: sans-serif;
3 | 	text-align: center;
4 | }


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/Cargo.toml:
--------------------------------------------------------------------------------
 1 | [workspace]
 2 | resolver = "2"
 3 | members = ["client", "api-boundary", "server"]
 4 | 
 5 | [profile.release]
 6 | codegen-units = 1
 7 | lto = true
 8 | 
 9 | [workspace.dependencies]
10 | api-boundary = { path = "api-boundary" }
11 | 
12 | [patch.crates-io]
13 | leptos = { path = "../../leptos" }
14 | leptos_router = { path = "../../router" }
15 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Login Example
 2 | 
 3 | This example demonstrates a scenario of a client-side rendered application
 4 | that uses an existing API that you cannot or do not want to change.
 5 | The authentications of this example are done using an API token.
 6 | 
 7 | The `api-boundary` crate contains data structures that are used by the server and the client.
 8 | 
 9 | ## Getting Started
10 | 
11 | See the [Examples README](../README.md) for setup and run instructions.
12 | 
13 | You will also need to run `cargo make stop` to end the server process.
14 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/api-boundary/Cargo.toml:
--------------------------------------------------------------------------------
1 | [package]
2 | name = "api-boundary"
3 | version = "0.0.0"
4 | edition = "2021"
5 | publish = false
6 | 
7 | [dependencies]
8 | serde = { version = "1.0", features = ["derive"] }
9 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/api-boundary/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../../cargo-make/main.toml" }
2 | 
3 | [tasks.check-format]
4 | env = { LEPTOS_PROJECT_DIRECTORY = "../../../" }
5 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/api-boundary/src/lib.rs:
--------------------------------------------------------------------------------
 1 | use serde::{Deserialize, Serialize};
 2 | 
 3 | #[derive(Serialize, Deserialize)]
 4 | pub struct Credentials {
 5 |     pub email: String,
 6 |     pub password: String,
 7 | }
 8 | 
 9 | #[derive(Clone, Serialize, Deserialize)]
10 | pub struct UserInfo {
11 |     pub email: String,
12 | }
13 | 
14 | #[derive(Clone, Serialize, Deserialize)]
15 | pub struct ApiToken {
16 |     pub token: String,
17 | }
18 | 
19 | #[derive(Debug, Serialize, Deserialize)]
20 | pub struct Error {
21 |     pub message: String,
22 | }
23 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/client/Makefile.toml:
--------------------------------------------------------------------------------
 1 | extend = [
 2 |     { path = "../../cargo-make/main.toml" },
 3 |     { path = "../../cargo-make/trunk_server.toml" },
 4 | ]
 5 | 
 6 | [env]
 7 | SERVER_PROCESS_NAME = "server"
 8 | 
 9 | [tasks.check-format]
10 | env = { LEPTOS_PROJECT_DIRECTORY = "../../../" }
11 | 
12 | [tasks.start-server]
13 | cwd = "../server"
14 | script = '''
15 | cargo run &
16 | '''
17 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/client/Trunk.toml:
--------------------------------------------------------------------------------
1 | [[proxy]]
2 | rewrite = "/api/"
3 | backend = "http://127.0.0.1:3000/"
4 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/client/index.html:
--------------------------------------------------------------------------------
1 | <!DOCTYPE html>
2 | <html>
3 | 	<head>
4 | 		<link data-trunk rel="rust" data-wasm-opt="z" data-weak-refs/>
5 | 	</head>
6 | 	<body></body>
7 | </html>
8 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/client/src/components/mod.rs:
--------------------------------------------------------------------------------
1 | pub mod credentials;
2 | pub mod navbar;
3 | 
4 | pub use self::navbar::*;
5 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/client/src/main.rs:
--------------------------------------------------------------------------------
1 | use client::*;
2 | use leptos::prelude::*;
3 | 
4 | pub fn main() {
5 |     _ = console_log::init_with_level(log::Level::Debug);
6 |     console_error_panic_hook::set_once();
7 |     mount_to_body(|| view! { <App/> })
8 | }
9 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/client/src/pages/mod.rs:
--------------------------------------------------------------------------------
 1 | pub mod home;
 2 | pub mod login;
 3 | pub mod register;
 4 | 
 5 | pub use self::{home::*, login::*, register::*};
 6 | 
 7 | #[derive(Debug, Clone, Copy, Default)]
 8 | pub enum Page {
 9 |     #[default]
10 |     Home,
11 |     Login,
12 |     Register,
13 | }
14 | 
15 | impl Page {
16 |     pub fn path(&self) -> &'static str {
17 |         match self {
18 |             Self::Home => "/",
19 |             Self::Login => "/login",
20 |             Self::Register => "/register",
21 |         }
22 |     }
23 | }
24 | 


--------------------------------------------------------------------------------
/projects/login_with_token_csr_only/server/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../../cargo-make/main.toml" }
2 | 
3 | [tasks.check-format]
4 | env = { LEPTOS_PROJECT_DIRECTORY = "../../../" }
5 | 


--------------------------------------------------------------------------------
/projects/meilisearch-searchbar/.gitignore:
--------------------------------------------------------------------------------
1 | /target
2 | meilisearch
3 | data.ms
4 | dumps
5 | Cargo.lock


--------------------------------------------------------------------------------
/projects/meilisearch-searchbar/Makefile.toml:
--------------------------------------------------------------------------------
1 | [tasks.ci]
2 | description = "Continuous Integration task"
3 | command = "cargo"
4 | args = ["test"] 
5 | 


--------------------------------------------------------------------------------
/projects/nginx-mpmc/.gitignore:
--------------------------------------------------------------------------------
1 | /target
2 | */target
3 | .vscode
4 | 


--------------------------------------------------------------------------------
/projects/nginx-mpmc/app-1/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | pkg
 5 | 
 6 | # These are backup files generated by rustfmt
 7 | **/*.rs.bk
 8 | 
 9 | # node e2e test tools and outputs
10 | node_modules/
11 | test-results/
12 | end2end/playwright-report/
13 | playwright/.cache/
14 | 


--------------------------------------------------------------------------------
/projects/nginx-mpmc/app-1/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/nginx-mpmc/app-1/public/favicon.ico


--------------------------------------------------------------------------------
/projects/nginx-mpmc/app-1/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod app;
 2 | pub mod error_template;
 3 | #[cfg(feature = "ssr")]
 4 | pub mod fileserv;
 5 | 
 6 | #[cfg(feature = "hydrate")]
 7 | #[wasm_bindgen::prelude::wasm_bindgen]
 8 | pub fn hydrate() {
 9 |     use crate::app::*;
10 |     console_error_panic_hook::set_once();
11 |     leptos::mount_to_body(App);
12 | }
13 | 


--------------------------------------------------------------------------------
/projects/nginx-mpmc/app-1/style/main.scss:
--------------------------------------------------------------------------------
1 | body {
2 | 	font-family: sans-serif;
3 | 	text-align: center;
4 | }


--------------------------------------------------------------------------------
/projects/nginx-mpmc/app-2/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | pkg
 5 | 
 6 | # These are backup files generated by rustfmt
 7 | **/*.rs.bk
 8 | 
 9 | # node e2e test tools and outputs
10 | node_modules/
11 | test-results/
12 | end2end/playwright-report/
13 | playwright/.cache/
14 | 


--------------------------------------------------------------------------------
/projects/nginx-mpmc/app-2/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/nginx-mpmc/app-2/public/favicon.ico


--------------------------------------------------------------------------------
/projects/nginx-mpmc/app-2/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod app;
 2 | pub mod error_template;
 3 | #[cfg(feature = "ssr")]
 4 | pub mod fileserv;
 5 | 
 6 | #[cfg(feature = "hydrate")]
 7 | #[wasm_bindgen::prelude::wasm_bindgen]
 8 | pub fn hydrate() {
 9 |     use crate::app::*;
10 |     console_error_panic_hook::set_once();
11 |     leptos::mount_to_body(App);
12 | }
13 | 


--------------------------------------------------------------------------------
/projects/nginx-mpmc/app-2/style/main.scss:
--------------------------------------------------------------------------------
1 | body {
2 | 	font-family: sans-serif;
3 | 	text-align: center;
4 | }


--------------------------------------------------------------------------------
/projects/nginx-mpmc/kill.sh:
--------------------------------------------------------------------------------
1 | lsof -ti :3000 | xargs kill && \
2 | lsof -ti :3001 | xargs kill &&  \
3 | lsof -ti :3002 | xargs kill && \
4 | lsof -ti :3003 | xargs kill && \
5 | lsof -ti :80 | xargs kill


--------------------------------------------------------------------------------
/projects/nginx-mpmc/run.sh:
--------------------------------------------------------------------------------
 1 | # save pwd variable
 2 | # append pwd to nginx.conf prefix
 3 | # run this command with the new nginx.conf path
 4 | (cd app-1 && cargo leptos serve)  & \
 5 | (cd app-2 && cargo leptos serve) & \
 6 | (cd shared-server-1 && cargo run) & \
 7 | (cd shared-server-2 && cargo run) & \
 8 | ( current_dir=$(pwd) && \
 9 | docker run --rm -v "$current_dir"/nginx.conf:/etc/nginx/nginx.conf:ro -p 80:80 nginx)
10 | 


--------------------------------------------------------------------------------
/projects/nginx-mpmc/run_linux.sh:
--------------------------------------------------------------------------------
 1 | # save pwd variable
 2 | # append pwd to nginx.conf prefix
 3 | # run this command with the new nginx.conf path
 4 | (cd app-1 && cargo leptos serve)  & \
 5 | (cd app-2 && cargo leptos serve) & \
 6 | (cd shared-server-1 && cargo run) & \
 7 | (cd shared-server-2 && cargo run) & \
 8 | ( current_dir=$(pwd) && \
 9 | docker run --rm -v "$current_dir"/nginx_linux.conf:/etc/nginx/nginx.conf:ro -p 80:80 --network="host" nginx)
10 | 


--------------------------------------------------------------------------------
/projects/nginx-mpmc/shared-server-1/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | pkg
 5 | 
 6 | # These are backup files generated by rustfmt
 7 | **/*.rs.bk
 8 | 
 9 | # node e2e test tools and outputs
10 | node_modules/
11 | test-results/
12 | end2end/playwright-report/
13 | playwright/.cache/
14 | 


--------------------------------------------------------------------------------
/projects/nginx-mpmc/shared-server-1/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/nginx-mpmc/shared-server-1/public/favicon.ico


--------------------------------------------------------------------------------
/projects/nginx-mpmc/shared-server-1/style/main.scss:
--------------------------------------------------------------------------------
1 | body {
2 | 	font-family: sans-serif;
3 | 	text-align: center;
4 | }


--------------------------------------------------------------------------------
/projects/nginx-mpmc/shared-server-2/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | pkg
 5 | 
 6 | # These are backup files generated by rustfmt
 7 | **/*.rs.bk
 8 | 
 9 | # node e2e test tools and outputs
10 | node_modules/
11 | test-results/
12 | end2end/playwright-report/
13 | playwright/.cache/
14 | 


--------------------------------------------------------------------------------
/projects/nginx-mpmc/shared-server-2/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/nginx-mpmc/shared-server-2/public/favicon.ico


--------------------------------------------------------------------------------
/projects/nginx-mpmc/shared-server-2/style/main.scss:
--------------------------------------------------------------------------------
1 | body {
2 | 	font-family: sans-serif;
3 | 	text-align: center;
4 | }


--------------------------------------------------------------------------------
/projects/openapi-openai-api-swagger-ui/.gitignore:
--------------------------------------------------------------------------------
 1 | # Generated by Cargo
 2 | # will have compiled files and executables
 3 | /target/
 4 | pkg
 5 | 
 6 | # These are backup files generated by rustfmt
 7 | **/*.rs.bk
 8 | 
 9 | # node e2e test tools and outputs
10 | node_modules/
11 | test-results/
12 | end2end/playwright-report/
13 | playwright/.cache/
14 | 
15 | .secret_key


--------------------------------------------------------------------------------
/projects/openapi-openai-api-swagger-ui/end2end/package.json:
--------------------------------------------------------------------------------
 1 | {
 2 |   "name": "end2end",
 3 |   "version": "1.0.0",
 4 |   "description": "",
 5 |   "main": "index.js",
 6 |   "scripts": {},
 7 |   "keywords": [],
 8 |   "author": "",
 9 |   "license": "ISC",
10 |   "devDependencies": {
11 |     "@playwright/test": "^1.28.0"
12 |   }
13 | }
14 | 


--------------------------------------------------------------------------------
/projects/openapi-openai-api-swagger-ui/end2end/tests/example.spec.ts:
--------------------------------------------------------------------------------
 1 | import { test, expect } from "@playwright/test";
 2 | 
 3 | test("homepage has title and links to intro page", async ({ page }) => {
 4 |   await page.goto("http://localhost:3000/");
 5 | 
 6 |   await expect(page).toHaveTitle("Welcome to Leptos");
 7 | 
 8 |   await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
 9 | });
10 | 


--------------------------------------------------------------------------------
/projects/openapi-openai-api-swagger-ui/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/openapi-openai-api-swagger-ui/public/favicon.ico


--------------------------------------------------------------------------------
/projects/openapi-openai-api-swagger-ui/style/main.scss:
--------------------------------------------------------------------------------
1 | body {
2 | 	font-family: sans-serif;
3 | 	text-align: center;
4 | }


--------------------------------------------------------------------------------
/projects/ory-kratos/.env:
--------------------------------------------------------------------------------
1 | DATABASE_URL="sqlite:app.db"


--------------------------------------------------------------------------------
/projects/ory-kratos/app/src/posts/mod.rs:
--------------------------------------------------------------------------------
1 | use super::*;
2 | mod post;
3 | use post::Post;
4 | pub mod posts_page;
5 | pub use posts_page::PostPage;
6 | mod create_posts;
7 | use crate::posts_page::PostData;
8 | use create_posts::CreatePost;
9 | 


--------------------------------------------------------------------------------
/projects/ory-kratos/e2e/features/0_test.feature:
--------------------------------------------------------------------------------
1 | @test
2 | Feature: Test
3 | 
4 |     Scenario:pass_test_pass
5 |         Given I pass
6 |   


--------------------------------------------------------------------------------
/projects/ory-kratos/frontend/src/lib.rs:
--------------------------------------------------------------------------------
 1 | use app::*;
 2 | use leptos::*;
 3 | use wasm_bindgen::prelude::wasm_bindgen;
 4 | 
 5 | #[wasm_bindgen]
 6 | pub fn hydrate() {
 7 |     // initializes logging using the `log` crate
 8 |     // _ = console_log::init_with_level(tracing::Level::Debug);
 9 |     console_error_panic_hook::set_once();
10 | 
11 |     leptos::mount_to_body(App);
12 | }
13 | 


--------------------------------------------------------------------------------
/projects/ory-kratos/ids/Cargo.toml:
--------------------------------------------------------------------------------
1 | [package]
2 | name = "ids"
3 | version = "0.1.0"
4 | edition = "2021"
5 | 
6 | [dependencies]
7 | 


--------------------------------------------------------------------------------
/projects/ory-kratos/migrations/01_create_users.sql:
--------------------------------------------------------------------------------
1 | CREATE TABLE users (
2 |         user_id TEXT PRIMARY KEY,
3 |         identity_id TEXT NOT NULL,
4 |         email TEXT NOT NULL
5 |     );
6 |  CREATE INDEX IF NOT EXISTS idx_identity_id ON users (identity_id);


--------------------------------------------------------------------------------
/projects/ory-kratos/migrations/02_create_posts.sql:
--------------------------------------------------------------------------------
1 | CREATE TABLE IF NOT EXISTS posts (
2 |         post_id TEXT PRIMARY KEY NOT NULL,
3 |         user_id TEXT NOT NULL,
4 |         content TEXT NOT NULL,
5 |         FOREIGN KEY (user_id) REFERENCES users(user_id)
6 |     );


--------------------------------------------------------------------------------
/projects/ory-kratos/migrations/03_create_post_permissions.sql:
--------------------------------------------------------------------------------
 1 | CREATE TABLE IF NOT EXISTS post_permissions (
 2 |         post_id TEXT NOT NULL,
 3 |         user_id TEXT NOT NULL,
 4 |         read BOOL NOT NULL,
 5 |         write BOOL NOT NULL,
 6 |         `delete` BOOL NOT NULL,
 7 |         FOREIGN KEY (user_id) REFERENCES users(user_id),
 8 |         FOREIGN KEY (post_id) REFERENCES posts(post_id),
 9 |         PRIMARY KEY (post_id, user_id)
10 |     );


--------------------------------------------------------------------------------
/projects/ory-kratos/public/apple_sso_btn.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/ory-kratos/public/apple_sso_btn.png


--------------------------------------------------------------------------------
/projects/ory-kratos/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/ory-kratos/public/favicon.ico


--------------------------------------------------------------------------------
/projects/session_auth_axum/.env:
--------------------------------------------------------------------------------
1 | DATABASE_URL=sqlite://Todos.db
2 | 


--------------------------------------------------------------------------------
/projects/session_auth_axum/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 |     { path = "../cargo-make/main.toml" },
3 |     { path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "session_auth_axum"
9 | 


--------------------------------------------------------------------------------
/projects/session_auth_axum/README.md:
--------------------------------------------------------------------------------
 1 | # Leptos Authenticated Todo App Sqlite with Axum
 2 | 
 3 | This example creates a basic todo app with an Axum backend that uses Leptos' server functions to call sqlx from the client and seamlessly run it on the server. It lets you login, signup, and submit todos as different users, or a guest.
 4 | 
 5 | ## Getting Started
 6 | 
 7 | See the [Examples README](../README.md) for setup and run instructions.
 8 | 
 9 | ## Quick Start
10 | 
11 | Run `cargo leptos watch` to run this example.
12 | 


--------------------------------------------------------------------------------
/projects/session_auth_axum/Todos.db:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/session_auth_axum/Todos.db


--------------------------------------------------------------------------------
/projects/session_auth_axum/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/session_auth_axum/public/favicon.ico


--------------------------------------------------------------------------------
/projects/session_auth_axum/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod auth;
 2 | pub mod error_template;
 3 | pub mod errors;
 4 | #[cfg(feature = "ssr")]
 5 | pub mod state;
 6 | pub mod todo;
 7 | 
 8 | #[cfg(feature = "hydrate")]
 9 | #[wasm_bindgen::prelude::wasm_bindgen]
10 | pub fn hydrate() {
11 |     use crate::todo::*;
12 |     _ = console_log::init_with_level(log::Level::Debug);
13 |     console_error_panic_hook::set_once();
14 | 
15 |     leptos::mount::hydrate_body(TodoApp);
16 | }
17 | 


--------------------------------------------------------------------------------
/projects/session_auth_axum/style.css:
--------------------------------------------------------------------------------
1 | .pending {
2 | 	color: purple;
3 | }
4 | 
5 | a {
6 | 	color: black;
7 | }


--------------------------------------------------------------------------------
/projects/sitemap_axum/.env.example:
--------------------------------------------------------------------------------
1 | POSTGRES_DB=blogs
2 | POSTGRES_USER=postgres
3 | POSTGRES_PASSWORD=password
4 | DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}?sslmode=disable
5 | 


--------------------------------------------------------------------------------
/projects/sitemap_axum/.gitignore:
--------------------------------------------------------------------------------
1 | .env
2 | Cargo.lock
3 | 


--------------------------------------------------------------------------------
/projects/sitemap_axum/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/sitemap_axum/public/favicon.ico


--------------------------------------------------------------------------------
/projects/sitemap_axum/src/lib.rs:
--------------------------------------------------------------------------------
 1 | pub mod app;
 2 | pub mod error_template;
 3 | #[cfg(feature = "ssr")]
 4 | pub mod fileserv;
 5 | #[cfg(feature = "ssr")]
 6 | pub mod sitemap;
 7 | 
 8 | #[cfg(feature = "hydrate")]
 9 | #[wasm_bindgen::prelude::wasm_bindgen]
10 | pub fn hydrate() {
11 |     use crate::app::*;
12 |     console_error_panic_hook::set_once();
13 |     leptos::mount_to_body(App);
14 | }
15 | 


--------------------------------------------------------------------------------
/projects/sso_auth_axum/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = [
2 | 	{ path = "../cargo-make/main.toml" },
3 | 	{ path = "../cargo-make/cargo-leptos.toml" },
4 | ]
5 | 
6 | [env]
7 | 
8 | CLIENT_PROCESS_NAME = "sso_auth_axum"
9 | 


--------------------------------------------------------------------------------
/projects/sso_auth_axum/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/sso_auth_axum/public/favicon.ico


--------------------------------------------------------------------------------
/projects/sso_auth_axum/src/state.rs:
--------------------------------------------------------------------------------
 1 | use axum::extract::FromRef;
 2 | use leptos::LeptosOptions;
 3 | use sqlx::SqlitePool;
 4 | 
 5 | /// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
 6 | /// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
 7 | #[derive(FromRef, Debug, Clone)]
 8 | pub struct AppState {
 9 |     pub leptos_options: LeptosOptions,
10 |     pub pool: SqlitePool,
11 |     pub client: oauth2::basic::BasicClient,
12 | }
13 | 


--------------------------------------------------------------------------------
/projects/sso_auth_axum/sso.db:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/sso_auth_axum/sso.db


--------------------------------------------------------------------------------
/projects/sso_auth_axum/style.css:
--------------------------------------------------------------------------------
1 | .pending {
2 | 	color: purple;
3 | }
4 | 
5 | a {
6 | 	color: black;
7 | }


--------------------------------------------------------------------------------
/projects/tauri-from-scratch/.gitignore:
--------------------------------------------------------------------------------
1 | /target
2 | 


--------------------------------------------------------------------------------
/projects/tauri-from-scratch/Cargo.toml:
--------------------------------------------------------------------------------
1 | [workspace]
2 | resolver = "2"
3 | members = ["src-tauri", "src-orig"]
4 | 
5 | [profile.release]
6 | codegen-units = 1
7 | lto = true
8 | 
9 | 


--------------------------------------------------------------------------------
/projects/tauri-from-scratch/Trunk.toml:
--------------------------------------------------------------------------------
1 | [build]
2 | target = "./src-orig/index.html"
3 | 
4 | [watch]
5 | ignore = ["./src-tauri"]


--------------------------------------------------------------------------------
/projects/tauri-from-scratch/public/favicon.ico:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/tauri-from-scratch/public/favicon.ico


--------------------------------------------------------------------------------
/projects/tauri-from-scratch/src-orig/index.html:
--------------------------------------------------------------------------------
 1 | <!DOCTYPE html>
 2 | <html>
 3 | 	<head>
 4 | 		<link data-trunk rel="rust" data-wasm-opt="z" data-bin="leptos_tauri_from_scratch_bin"/>
 5 | 		<link rel="icon" type="image/x-icon" href="favicon.ico">
 6 | 		<meta charset="UTF-8" />
 7 | 		<meta name="viewport" content="width=device-width, initial-scale=1.0" />
 8 | 	</head>
 9 | 	<body></body>
10 | </html>


--------------------------------------------------------------------------------
/projects/tauri-from-scratch/src-tauri/build.rs:
--------------------------------------------------------------------------------
1 | fn main() {
2 |     tauri_build::build();
3 | }
4 | 


--------------------------------------------------------------------------------
/projects/tauri-from-scratch/src-tauri/icons/icon.png:
--------------------------------------------------------------------------------
https://raw.githubusercontent.com/leptos-rs/leptos/f4e0be2d5954f1ddeb0f93cd6c229a682942d052/projects/tauri-from-scratch/src-tauri/icons/icon.png


--------------------------------------------------------------------------------
/projects/tauri-from-scratch/src-tauri/src/lib.rs:
--------------------------------------------------------------------------------
 1 | use tauri::Manager;
 2 | 
 3 | #[cfg_attr(mobile, tauri::mobile_entry_point)]
 4 | pub fn run() {
 5 |     tauri::Builder::default()
 6 |         .plugin(tauri_plugin_http::init())
 7 |         .setup(|app| {
 8 |             {
 9 |                 let window = app.get_window("main").unwrap();
10 |                 window.open_devtools();
11 |             }
12 |             Ok(())
13 |         })
14 |         .run(tauri::generate_context!())
15 |         .expect("error while running tauri application");
16 | }


--------------------------------------------------------------------------------
/projects/tauri-from-scratch/src-tauri/src/main.rs:
--------------------------------------------------------------------------------
1 | // Prevents additional console window on Windows in release, DO NOT REMOVE!!
2 | #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
3 | 
4 | fn main() {
5 |     app_lib::run();
6 | }


--------------------------------------------------------------------------------
/reactive_graph/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/reactive_graph/build.rs:
--------------------------------------------------------------------------------
1 | use rustc_version::{version_meta, Channel};
2 | 
3 | fn main() {
4 |     // Set cfg flags depending on release channel
5 |     if matches!(version_meta().unwrap().channel, Channel::Nightly) {
6 |         println!("cargo:rustc-cfg=rustc_nightly");
7 |     }
8 | }
9 | 


--------------------------------------------------------------------------------
/reactive_graph/src/actions/mod.rs:
--------------------------------------------------------------------------------
1 | //! Reactive primitives to asynchronously update some value.
2 | 
3 | mod action;
4 | mod multi_action;
5 | pub use action::*;
6 | pub use multi_action::*;
7 | 


--------------------------------------------------------------------------------
/reactive_graph/src/graph.rs:
--------------------------------------------------------------------------------
 1 | //! Types that define the reactive graph itself. These are mostly internal, but can be used to
 2 | //! create custom reactive primitives.
 3 | 
 4 | mod node;
 5 | mod sets;
 6 | mod source;
 7 | mod subscriber;
 8 | 
 9 | pub use node::*;
10 | pub(crate) use sets::*;
11 | pub use source::*;
12 | pub use subscriber::*;
13 | 


--------------------------------------------------------------------------------
/reactive_stores/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/reactive_stores_macro/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/reactive_stores_macro/README.md:
--------------------------------------------------------------------------------
1 | This crate provides macro that are helpful or required when using the `reactive_stores` crate.
2 | 


--------------------------------------------------------------------------------
/router/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/router/build.rs:
--------------------------------------------------------------------------------
1 | use rustc_version::{version_meta, Channel};
2 | 
3 | fn main() {
4 |     // Set cfg flags depending on release channel
5 |     if matches!(version_meta().unwrap().channel, Channel::Nightly) {
6 |         println!("cargo:rustc-cfg=rustc_nightly");
7 |     }
8 | }
9 | 


--------------------------------------------------------------------------------
/router/src/matching/vertical/mod.rs:
--------------------------------------------------------------------------------
1 | use super::PartialPathMatch;
2 | 
3 | pub trait ChooseRoute {
4 |     fn choose_route<'a>(&self, path: &'a str) -> Option<PartialPathMatch<'a>>;
5 | }
6 | 


--------------------------------------------------------------------------------
/router_macro/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/rustfmt.toml:
--------------------------------------------------------------------------------
 1 | # Stable options
 2 | edition = "2021"
 3 | max_width = 80
 4 | 
 5 | # Unstable options
 6 | imports_granularity = "Crate"
 7 | format_strings = true
 8 | group_imports = "One"
 9 | format_code_in_doc_comments = true
10 | 


--------------------------------------------------------------------------------
/server_fn/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/server_fn/build.rs:
--------------------------------------------------------------------------------
1 | use rustc_version::{version_meta, Channel};
2 | 
3 | fn main() {
4 |     // Set cfg flags depending on release channel
5 |     if matches!(version_meta().unwrap().channel, Channel::Nightly) {
6 |         println!("cargo:rustc-cfg=rustc_nightly");
7 |     }
8 | }
9 | 


--------------------------------------------------------------------------------
/server_fn/server_fn_macro_default/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../../cargo-make/main.toml" }
2 | 
3 | [tasks.check-format]
4 | env = { LEPTOS_PROJECT_DIRECTORY = "../../" }
5 | 


--------------------------------------------------------------------------------
/server_fn/tests/invalid/aliased_return_full.rs:
--------------------------------------------------------------------------------
 1 | use server_fn_macro_default::server;
 2 | 
 3 | #[derive(Debug, thiserror::Error, Clone, serde::Serialize, serde::Deserialize)]
 4 | pub enum InvalidError {
 5 |     #[error("error a")]
 6 |     A,
 7 | }
 8 | 
 9 | type FullAlias = Result<String, InvalidError>;
10 | 
11 | #[server]
12 | pub async fn full_alias_result() -> FullAlias {
13 |     Ok("hello".to_string())
14 | }
15 | 
16 | fn main() {}


--------------------------------------------------------------------------------
/server_fn/tests/invalid/aliased_return_none.rs:
--------------------------------------------------------------------------------
 1 | use server_fn_macro_default::server;
 2 | 
 3 | #[derive(Debug, thiserror::Error, Clone, serde::Serialize, serde::Deserialize)]
 4 | pub enum InvalidError {
 5 |     #[error("error a")]
 6 |     A,
 7 | }
 8 | 
 9 | #[server]
10 | pub async fn no_alias_result() -> Result<String, InvalidError> {
11 |     Ok("hello".to_string())
12 | }
13 | 
14 | fn main() {}


--------------------------------------------------------------------------------
/server_fn/tests/invalid/aliased_return_part.rs:
--------------------------------------------------------------------------------
 1 | use server_fn_macro_default::server;
 2 | 
 3 | #[derive(Debug, thiserror::Error, Clone, serde::Serialize, serde::Deserialize)]
 4 | pub enum InvalidError {
 5 |     #[error("error a")]
 6 |     A,
 7 | }
 8 | 
 9 | type PartAlias<T> = Result<T, InvalidError>;
10 | 
11 | #[server]
12 | pub async fn part_alias_result() -> PartAlias<String> {
13 |     Ok("hello".to_string())
14 | }
15 | 
16 | fn main() {}


--------------------------------------------------------------------------------
/server_fn/tests/invalid/empty_return.rs:
--------------------------------------------------------------------------------
1 | use server_fn_macro_default::server;
2 | 
3 | #[server]
4 | pub async fn empty_return() -> () {
5 |     ()
6 | }
7 | 
8 | fn main() {}


--------------------------------------------------------------------------------
/server_fn/tests/invalid/no_return.rs:
--------------------------------------------------------------------------------
1 | use server_fn_macro_default::server;
2 | 
3 | #[server]
4 | pub async fn no_return() {}
5 | 
6 | fn main() {}


--------------------------------------------------------------------------------
/server_fn/tests/invalid/no_return.stderr:
--------------------------------------------------------------------------------
1 | error: expected `->`
2 |  --> tests/invalid/no_return.rs:4:26
3 |   |
4 | 4 | pub async fn no_return() {}
5 |   |                          ^
6 | 


--------------------------------------------------------------------------------
/server_fn/tests/invalid/not_async.rs:
--------------------------------------------------------------------------------
1 | use server_fn_macro_default::server;
2 | use server_fn::error::ServerFnError;
3 | 
4 | #[server]
5 | pub fn not_async() -> Result<String, ServerFnError> {
6 |     Ok("hello".to_string())
7 | }
8 | 
9 | fn main() {}


--------------------------------------------------------------------------------
/server_fn/tests/invalid/not_async.stderr:
--------------------------------------------------------------------------------
 1 | error: expected `async`
 2 |  --> tests/invalid/not_async.rs:5:5
 3 |   |
 4 | 5 | pub fn not_async() -> Result<String, ServerFnError> {
 5 |   |     ^^
 6 | 
 7 | warning: unused import: `server_fn::error::ServerFnError`
 8 |  --> tests/invalid/not_async.rs:2:5
 9 |   |
10 | 2 | use server_fn::error::ServerFnError;
11 |   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
12 |   |
13 |   = note: `#[warn(unused_imports)]` on by default
14 | 


--------------------------------------------------------------------------------
/server_fn/tests/valid/aliased_return_full.rs:
--------------------------------------------------------------------------------
 1 | use server_fn_macro_default::server;
 2 | use server_fn::error::ServerFnError;
 3 | 
 4 | type FullAlias = Result<String, ServerFnError>;
 5 | 
 6 | #[server]
 7 | pub async fn full_alias_result() -> FullAlias {
 8 |     Ok("hello".to_string())
 9 | }
10 | 
11 | fn main() {}


--------------------------------------------------------------------------------
/server_fn/tests/valid/aliased_return_none.rs:
--------------------------------------------------------------------------------
 1 | use server_fn_macro_default::server;
 2 | use server_fn::error::ServerFnError;
 3 | 
 4 | #[server]
 5 | pub async fn no_alias_result() -> Result<String, ServerFnError> {
 6 |     Ok("hello".to_string())
 7 | }
 8 | 
 9 | 
10 | fn main() {}


--------------------------------------------------------------------------------
/server_fn/tests/valid/aliased_return_part.rs:
--------------------------------------------------------------------------------
 1 | use server_fn_macro_default::server;
 2 | use server_fn::error::ServerFnError;
 3 | 
 4 | type PartAlias<T> = Result<T, ServerFnError>;
 5 | 
 6 | #[server]
 7 | pub async fn part_alias_result() -> PartAlias<String> {
 8 |     Ok("hello".to_string())
 9 | }
10 | 
11 | fn main() {}


--------------------------------------------------------------------------------
/server_fn_macro/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/server_fn_macro/build.rs:
--------------------------------------------------------------------------------
1 | use rustc_version::{version_meta, Channel};
2 | 
3 | fn main() {
4 |     // Set cfg flags depending on release channel
5 |     if matches!(version_meta().unwrap().channel, Channel::Nightly) {
6 |         println!("cargo:rustc-cfg=rustc_nightly");
7 |     }
8 | }
9 | 


--------------------------------------------------------------------------------
/tachys/Makefile.toml:
--------------------------------------------------------------------------------
1 | extend = { path = "../cargo-make/main.toml" }
2 | 


--------------------------------------------------------------------------------
/tachys/build.rs:
--------------------------------------------------------------------------------
1 | use rustc_version::{version_meta, Channel};
2 | 
3 | fn main() {
4 |     // Set cfg flags depending on release channel
5 |     if matches!(version_meta().unwrap().channel, Channel::Nightly) {
6 |         println!("cargo:rustc-cfg=rustc_nightly");
7 |     }
8 | }
9 | 


--------------------------------------------------------------------------------