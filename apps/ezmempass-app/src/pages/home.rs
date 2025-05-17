/*
 * Home page component for the EzMemPass application.
 */

use leptos::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::password_generator::PasswordGenerator;

/// Home page component
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="app-container">
            <Navbar active="home" />

            <div class="container">
                <div class="hero">
                    <h1>"EzMemPass"</h1>
                    <h2>"Generate Strong, Memorable Passwords"</h2>
                    <p>
                        "Create secure passwords that are easy to remember using advanced language models and semantic connections."
                    </p>
                </div>

                <div class="main-content">
                    <PasswordGenerator />
                </div>

                <div class="features">
                    <div class="feature-card">
                        <h3>"Memorable"</h3>
                        <p>"Words connected by meaning are easier to remember than random characters."</p>
                    </div>
                    <div class="feature-card">
                        <h3>"Secure"</h3>
                        <p>"All passwords provide high entropy for strong security."</p>
                    </div>
                    <div class="feature-card">
                        <h3>"Private"</h3>
                        <p>"All generation happens locally on your device - no data leaves your browser."</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
