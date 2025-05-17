/*
 * Navigation bar component for the EzMemPass application.
 */

use leptos::prelude::*;
use leptos_router::components::A;

/// Navigation bar component
#[component]
pub fn Navbar(
    /// The currently active page
    #[prop(into)]
    active: &'static str,
) -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-brand">
                <A href="/">"EzMemPass"</A>
            </div>
            <div class="navbar-links">
                <A href="/"          class:active=move || active == "home">     "Home"     </A>
                <A href="/settings"  class:active=move || active == "settings"> "Settings" </A>
                <A href="/help"      class:active=move || active == "help">     "Help"     </A>
            </div>
        </nav>
    }
}
