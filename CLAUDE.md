# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Leptos Client-Side Rendered (CSR) web application built with Rust and WebAssembly. The project is a landing page for "NSup" - a manufacturing efficiency platform that provides tool management, NC program management, and AI-powered assistance for manufacturing operations.

## Key Technologies

- **Leptos 0.8**: Rust web framework with CSR features
- **Trunk**: Build tool and development server for Rust/WASM applications
- **TailwindCSS**: Utility-first CSS framework for styling
- **WebAssembly**: Target compilation for browser execution

## Development Commands

### Development Server
```bash
trunk serve --port 3000 --open
```
Starts development server with hot reload at http://localhost:3000

### Production Build
```bash
trunk build --release
```
Builds optimized production assets to `dist/` directory

### Testing
```bash
cargo test                    # Run all tests
cargo test <test_name>        # Run specific test
wasm-pack test --headless --firefox  # Run WASM tests in browser
```

### CSS Build (Development)
```bash
npm run build-css
```
Builds TailwindCSS with watch mode to `dist/tailwind.css`

### CSS Build (Production)
```bash
npm run build-css-prod
```
Builds minified TailwindCSS for production to `dist/tailwind.css`

### Code Quality
```bash
cargo fmt                     # Format code
cargo clippy                  # Run linter
cargo check                   # Check compilation without building
```

## Architecture

### Project Structure
- `src/lib.rs`: Main app component with router setup and all route definitions
- `src/main.rs`: Entry point that mounts the app to DOM
- `src/pages/`: Page components for each route (8 total pages)
- `src/components/`: Reusable UI components for landing page and shared elements
- `src/utils/`: Utility modules for animations and event handlers
- `index.html`: HTML template that Trunk uses for bundling
- `Trunk.toml`: Trunk configuration for build and serve settings
- `input.css`: TailwindCSS source file
- `tailwind.config.js`: TailwindCSS configuration

### Component Architecture
The application uses Leptos's component system with:
- Functional components using the `#[component]` macro
- Reactive signals for state management
- JSX-like view! macro for templating
- Client-side routing with leptos_router

### Router Configuration  
The app uses leptos_router with the following routes configured in `src/lib.rs`:
- `/` - Home landing page
- `/tools` - Tool management system
- `/employees` - Employee management system  
- `/nc-programs` - NC program management
- `/nc-support` - AI-powered NC program support
- `/ai-suggestions` - AI tool optimization suggestions
- `/chat` - Team communication chat
- Fallback: 404 NotFound component

### Page Architecture
This is a multi-page application with the following main pages:
- `src/pages/home.rs`: Landing page with N-Sup platform overview
- `src/pages/tool_management.rs`: Tool inventory and management interface
- `src/pages/employee_management.rs`: Employee database and status management
- `src/pages/nc_program_management.rs`: NC program version control and status tracking
- `src/pages/nc_program_support.rs`: AI-powered NC program optimization and generation
- `src/pages/ai_tool_suggestions.rs`: AI-driven manufacturing improvement suggestions
- `src/pages/chat.rs`: Real-time team communication system
- `src/pages/not_found.rs`: 404 error page

### Component Architecture
Components are organized into reusable modules:
- `src/components/header.rs`: Navigation header with multi-page menu
- `src/components/hero_section.rs`: Landing page hero section
- `src/components/features.rs`: Feature showcase cards
- `src/components/benefits.rs`: Benefits presentation section
- `src/components/cta_section.rs`: Call-to-action section
- `src/components/footer.rs`: Site footer
- `src/components/ui.rs`: Common UI elements and utilities
- `src/components/counter_btn.rs`: Unused example component

### Important Implementation Notes  
- Each page is a standalone component with its own functionality
- All styling uses Tailwind utility classes with consistent dark theme design
- The application focuses on manufacturing efficiency and AI-powered optimization
- Components are properly exported through mod.rs files for clean module organization

## Setup Requirements

This project uses Rust stable toolchain and requires:
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`
- Node.js/npm for TailwindCSS

## Styling

The project uses TailwindCSS for styling with a dark theme featuring:
- Gradient backgrounds (slate-900 to slate-800)
- Blue to purple gradient accents
- Responsive design with mobile-first approach
- Glass morphism effects with backdrop-blur

CSS is processed from `input.css` and output to `dist/tailwind.css` via the npm scripts.

## Module Organization

### Page Modules
Each page module is self-contained and exports its main component:
- All pages follow the same pattern: `#[component] pub fn PageName() -> impl IntoView`
- Pages handle their own state management and business logic
- Import shared components from `src/components` as needed

### Component Modules  
Components are organized for reusability:
- `mod.rs` files handle module exports and re-exports
- Common UI patterns are abstracted into `src/components/ui.rs`
- Landing page components are modular and can be reused across pages

### Utility Modules
- `src/utils/animations.rs`: Animation and transition utilities
- `src/utils/event_handlers.rs`: Common event handling functions
- Utilities are designed to work with Leptos's reactive system

## Leptos Development Patterns

**IMPORTANT**: When writing Leptos code, always reference `LEPTOS.md` for comprehensive documentation and examples.

### Component Patterns
```rust
#[component]
pub fn ComponentName() -> impl IntoView {
    view! {
        // JSX-like syntax
    }
}
```

### State Management
- Use `signal()` for reactive state: `let (getter, setter) = signal(initial_value)`
- Use `Effect::new()` for side effects and DOM manipulation
- Keep state local to components when possible

### Props and Composition
- Use `#[prop(into)]` for automatic type conversion
- Use `children: Children` for component composition
- Use `#[prop(optional)]` for optional props

### Event Handling
- Use `wasm_bindgen` closures for DOM event listeners
- Pattern: `closure.forget()` for persistent listeners
- Use move closures for reactive class binding

### Import Conventions
- Use `leptos::prelude::*` for core functionality
- Import specific modules as needed: `leptos_meta::*`, `leptos_router::*`