use ezmempass_core::{generator::PasswordGeneratorFactory, types::*};
use futures::{sink::SinkExt, StreamExt};
use gloo_worker::reactor::{reactor, ReactorScope};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Request {
    GeneratePassword(GenerationOptions),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Response {
    GeneratedPassword(GeneratedPassword),
}

#[reactor]
pub async fn EzMemPassWorker(mut scope: ReactorScope<Request, Response>) {
    log::info!("EzMemPassWorkerReactor function triggered");
    while let Some(m) = scope.next().await {
        match m {
            Request::GeneratePassword(opts) => {
                log::info!(
                    "EzMemPassWorkerReactor processing message. opts: {:?}",
                    opts
                );

                let result = PasswordGeneratorFactory::create(opts.preferred_method)
                    .generate(&opts)
                    .unwrap();

                log::info!("EzMemPassWorkerReactor finished processing message, sending result");

                if scope
                    .send(Response::GeneratedPassword(result))
                    .await
                    .is_err()
                {
                    log::error!("EzMemPassWorkerReactor failed to send result");
                    break;
                }

                log::info!("EzMemPassWorkerReactor sent result");
            }
        }
    }
}
