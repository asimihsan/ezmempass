/*
 * Password generator component for the EzMemPass application.
 */
use ezmempass_core::types::GenerationOptions;
use ezmempass_worker::{EzMemPassWorker, Request, Response};
use gloo_worker::oneshot;
use leptos::prelude::*;

/// Password generator component
#[component]
pub fn PasswordGenerator() -> impl IntoView {
    let (options, _set_options) = signal(GenerationOptions::default());

    let generate = Action::new(move |_: &()| {
        let opts = options.get_untracked();
        async move {
            // send request and await response
            let Response::Generated(gp) =
                oneshot::oneshot::<EzMemPassWorker>(Request::Generate(opts))
                    .await
                    .unwrap();
            gp
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
