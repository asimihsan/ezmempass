/*
 * 404 Not Found page component for the EzMemPass application.
 */

use leptos::prelude::*;
use leptos_router::components::A;

/// 404 Not Found page component
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="container not-found">
            <h1>"404 - Page Not Found"</h1>
            <div class="card">
                <p>"The page you are looking for does not exist or has been moved."</p>
                <A href="/" class:button=true>
                    "Return to Home"
                </A>
            </div>
        </div>
    }
}
