/*
 * Home page component for the EzMemPass application.
 */

use crate::components::password_generator::PasswordGenerator;
use leptos::prelude::*;

/// Home page component
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-svh">
            <main class="container mx-auto px-4 grow">
                <section class="mx-auto my-16 max-w-3xl text-center space-y-4">
                    <h1 class="text-4xl font-extrabold tracking-tight">"EzMemPass"</h1>
                    <h2 class="text-xl text-gray-600">"Generate Strong, Memorable Passwords"</h2>
                    <p class="text-lg text-gray-700">
                        "Create secure passwords that are easy to remember using advanced language models and semantic connections."
                    </p>
                </section>

                <section class="grid gap-8 lg:grid-cols-2">
                    <PasswordGenerator />
                </section>

                <section class="grid gap-6 my-20 sm:grid-cols-2 lg:grid-cols-3">
                    <div class="card text-center space-y-2">
                        <h3 class="text-lg font-semibold">"Memorable"</h3>
                        <p>
                            "Words connected by meaning are easier to remember than random characters."
                        </p>
                    </div>
                </section>
            </main>
        </div>
    }
}
