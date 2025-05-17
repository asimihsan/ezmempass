use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
pub mod components;
mod pages;

// Top-Level pages
use crate::pages::help::HelpPage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::settings::SettingsPage;

/// App root component
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="EzMemPass - Memorable Password Generator" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Meta name="description" content="Generate strong, memorable passwords" />

        <Router>
            <main>
                <Routes fallback=|| view! { <NotFoundPage /> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/settings") view=SettingsPage />
                    <Route path=path!("/help") view=HelpPage />
                </Routes>
            </main>
        </Router>
    }
}
