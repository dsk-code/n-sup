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
Builds TailwindCSS with watch mode

### CSS Build (Production)
```bash
npm run build-css-prod
```
Builds minified TailwindCSS for production

### Code Quality
```bash
cargo fmt                     # Format code
cargo clippy                  # Run linter
cargo check                   # Check compilation without building
```

## Architecture

### Project Structure
- `src/lib.rs`: Main app component with router setup
- `src/main.rs`: Entry point that mounts the app
- `src/pages/`: Page components (currently just home.rs)
- `src/components/`: Reusable UI components
- `index.html`: HTML template that Trunk uses for bundling
- `Trunk.toml`: Trunk configuration for build and serve settings

### Component Architecture
The application uses Leptos's component system with:
- Functional components using the `#[component]` macro
- Reactive signals for state management
- JSX-like view! macro for templating
- Client-side routing with leptos_router

### Router Configuration
The app uses leptos_router with:
- Main route `/` mapped to `Home` component
- 404 fallback handled by `NotFound` component  
- Router configured in `src/lib.rs` with fallback pattern

### Page Architecture
- `src/pages/home.rs`: Complete landing page with embedded components (Header, Hero, Features, Benefits, CTA, Footer)
- `src/pages/not_found.rs`: Simple 404 error page
- Components are defined inline within home.rs rather than as separate files

### Important Implementation Notes
- The home page contains multiple inline components that could be extracted to `src/components/` for reusability
- Current counter_btn.rs component is commented out and unused
- All styling uses Tailwind utility classes with consistent design system

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