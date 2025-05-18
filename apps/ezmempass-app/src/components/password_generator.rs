/*
 * Password generator component for the EzMemPass application.
 */
use ezmempass_core::types::GenerationOptions;
use ezmempass_worker::Response;
use ezmempass_worker::{EzMemPassWorker, Request};
use futures::StreamExt;
use gloo_worker::{Registrable, Spawnable};
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Password generator component
#[component]
pub fn PasswordGenerator() -> impl IntoView {
    log::info!("PasswordGenerator component running");

    let (options, _set_options) = signal(GenerationOptions::default());

    EzMemPassWorker::registrar().register();
    let root_bridge = Rc::new(RefCell::new(
        EzMemPassWorker::spawner().spawn("/workers/ezmempass-worker.js"),
    ));

    // Trigger once on mount
    let generate = {
        let root_bridge = Rc::clone(&root_bridge);

        Action::new_local(move |_| {
            log::info!("PasswordGenerator action triggered");
            let root_bridge = Rc::clone(&root_bridge);
            log::info!("PasswordGenerator action sending message to worker");
            async move {
                log::info!("PasswordGenerator action sending message to worker");
                let mut my_bridge = {
                    // immutable borrow ends right after we fork
                    let parent = root_bridge.borrow();
                    parent.fork()
                };

                my_bridge.send_input(Request::GeneratePassword(options.get_untracked().clone()));
                log::info!("PasswordGenerator action sent message to worker");

                // Wait for the response from the worker
                match my_bridge.next().await {
                    Some(Response::GeneratedPassword(pw)) => pw,
                    _ => unreachable!("worker closed without reply"),
                }
            }
        })
    };

    // Trigger the action to generate a password on mount
    generate.dispatch(());

    let generate_pending = generate.pending();
    let generate_value = generate.value();
    let root_bridge_for_click = Rc::clone(&root_bridge);

    view! {
        <div class="password-generator">
            <Show
                when=move || !generate_pending.get()
                fallback=|| view! { <span>"Generating..."</span> }
            >
                {move || {
                    let gp = generate_value.get().unwrap();
                    view! {
                        <div>
                            <h2>"Generated Password"</h2>
                            <p>{format!("Password: {}", gp.password)}</p>
                            <p>{format!("Entropy Bits: {:.2}", gp.entropy_bits)}</p>
                            <p>{format!("Method: {}", gp.method)}</p>
                        </div>
                    }
                }}

            </Show>

            <button
                class="btn-primary"
                on:click=move |_| {
                    let click_bridge = {
                        let parent = root_bridge_for_click.borrow();
                        parent.fork()
                    };

                    click_bridge.send_input(Request::GeneratePassword(
                        options.get().clone(),
                    ));
                }
            >
                "Generate New"
            </button>
        </div>
    }
}
