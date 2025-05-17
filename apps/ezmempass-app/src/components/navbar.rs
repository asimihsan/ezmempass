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
    active: Signal<&'static str>,
) -> impl IntoView {
    view! {
        <nav class="flex items-center justify-between h-16 px-8 bg-[--surface-2] shadow">
            <A href="/" attr:class="text-xl font-bold text-[--brand]">
                "EzMemPass"
            </A>

            <div class="flex gap-6 text-gray-700">
                <A
                    href="/"
                    // turn the underline utilities on only when this is the current page
                    class:nav-active=move || active.get() == "home"
                >
                    "Home"
                </A>
                <a
                    href="/settings"
                    class=(
                        "after:content-[''] after:w-full after:h-0.5 after:absolute after:-bottom-1 after:left-0 after:bg-[--brand]",
                        move || active.get() == "settings",
                    )
                >
                    "Settings"
                </a>
                <a
                    href="/help"
                    class=(
                        "after:content-[''] after:w-full after:h-0.5 after:absolute after:-bottom-1 after:left-0 after:bg-[--brand]",
                        move || active.get() == "help",
                    )
                >
                    "Help"
                </a>
            </div>
        </nav>
    }
}
