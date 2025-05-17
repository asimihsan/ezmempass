use ezmempass_core::{generator::PasswordGeneratorFactory, types::*};
use gloo_worker::oneshot::oneshot;

#[oneshot]
async fn EzMemPassWorker(opts: GenerationOptions) -> GeneratedPassword {
    PasswordGeneratorFactory::create(opts.preferred_method)
        .generate(&opts)
        .unwrap()
}
