/*
 * Password generator component for the EzMemPass application.
 */

use ezmempass_core::generator::PasswordGeneratorFactory;
use ezmempass_core::types::GenerationOptions;
use leptos::prelude::*;

/// Password generator component
#[component]
pub fn PasswordGenerator() -> impl IntoView {
    let (options, set_options) = signal(GenerationOptions::default());

    let generate = Action::new(move |_: &()| {
        let opts = options.get_untracked();
        // run blocking code inside `spawn_local` so we don't choke the event loop
        async move {
            let generator = PasswordGeneratorFactory::create(opts.preferred_method);
            generator.generate(&opts).unwrap()
        }
    });

    // Trigger once on mount
    generate.dispatch(());

    // Read the result
    let password = move || generate.value().get().map(|gp| gp.password.clone());

    view! {
        <div class="password-generator">
            <Show
                when=move || password().is_some()
                fallback=|| view! { <span>"Generating..."</span> }
            >
                {move || password().unwrap()}
            </Show>

            <button
                class="btn-primary"
                on:click=move |_| {
                    generate.dispatch(());
                }
            >
                "Generate New"
            </button>
        </div>
    }
}
