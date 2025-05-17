use crate::components::navbar::Navbar;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Top-Level pages
use crate::pages::help::HelpPage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::settings::SettingsPage;

/// App root component
#[component]
pub fn Shell() -> impl IntoView {
    // compute active page from the browser location
    let location = leptos_router::hooks::use_location();
    let active = Signal::derive(move || match location.pathname.get().as_str() {
        "/" => "home",
        "/settings" => "settings",
        "/help" => "help",
        _ => "",
    });

    view! {
        <Navbar active=active />
        <main>
            <Routes fallback=|| view! { <NotFoundPage /> }>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/settings") view=SettingsPage />
                <Route path=path!("/help") view=HelpPage />
            </Routes>
        </main>
    }
}
