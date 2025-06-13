use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod pages;
mod utils;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::dashboard::Dashboard;
use crate::pages::nc_program_management::NcProgramManagement;
use crate::pages::chat::Chat;
use crate::pages::tool_management::ToolManagement;
use crate::pages::employee_management::EmployeeManagement;
use crate::pages::ai_tool_suggestions::AiToolSuggestions;
use crate::pages::nc_program_support::NcProgramSupport;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Welcome to Leptos CSR" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/dashboard") view=Dashboard />
                <Route path=path!("/tools") view=ToolManagement />
                <Route path=path!("/employees") view=EmployeeManagement />
                <Route path=path!("/nc-programs") view=NcProgramManagement />
                <Route path=path!("/chat") view=Chat />
                <Route path=path!("/ai-suggestions") view=AiToolSuggestions />
                <Route path=path!("/nc-support") view=NcProgramSupport />
            </Routes>
        </Router>
    }
}
