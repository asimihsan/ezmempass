/*
 * Password generator component for the EzMemPass application.
 */
use ezmempass_core::types::GenerationOptions;
use ezmempass_worker::Response;
use ezmempass_worker::{EzMemPassWorker, Request};
use futures::{sink::SinkExt, StreamExt};
use gloo_worker::{Registrable, Spawnable};
use leptos::prelude::*;

/// Password generator component
#[component]
pub fn PasswordGenerator() -> impl IntoView {
    log::info!("PasswordGenerator component running");

    let (options, _set_options) = signal(GenerationOptions::default());

    EzMemPassWorker::registrar().register();

    let mut bridge = EzMemPassWorker::spawner().spawn("/workers/ezmempass-worker.js");

    // Trigger once on mount
    let generate = Action::new_local(move |_| {
        log::info!("PasswordGenerator sending message to worker");
        async move {
            log::info!("PasswordGenerator sending message to worker");
            bridge.send_input(Request::GeneratePassword(options.get_untracked().clone()));

            let result = bridge.next().await.unwrap();
            log::info!("PasswordGenerator received result: {:?}", result);

            match result {
                Response::GeneratedPassword(gp) => {
                    log::info!("PasswordGenerator setting password: {:?}", gp);
                    return Some(gp);
                }
            }
        }
    });

    let generate_pending = generate.pending();
    let generate_value = generate.value();

    // Trigger the action to generate a password on mount
    generate.dispatch(());

    view! {
        <div class="password-generator">
            <Show
                when=move || !generate_pending.get()
                fallback=|| view! { <span>"Generating..."</span> }
            >
                {move || {
                    view! {
                        <div>
                            <h2>"Generated Password"</h2>
                            <p>{format!("Password: {}", generate_value.get().unwrap().unwrap().password)}</p>
                            <p>{format!("Entropy Bits: {:.2}", generate_value.get().unwrap().unwrap().entropy_bits)}</p>
                            <p>{format!("Method: {}", generate_value.get().unwrap().unwrap().method)}</p>
                        </div>
                    }
                }}

            </Show>

            <button
                class="btn-primary"
                on:click=move |_| {
                    bridge.send(Request::GeneratePassword(options.get().clone()));
                }
            >
                "Generate New"
            </button>
        </div>
    }
}
