use crate::{commands::slash::ping, constants};
pub fn create() -> poise::Framework<
    constants::Data,
    Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>,
> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping::ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(constants::Data {})
            })
        })
        .build()
}
