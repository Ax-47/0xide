use crate::{
    commands::slash::{config, ping},
    constants,
    events::handle,
    infrastructure::services::guild,
};
pub async fn create(
    uri: String,
) -> poise::Framework<
    constants::Data,
    Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>,
> {
    let guild_service = guild::GuildService::new(uri).await.unwrap();
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping::ping(), config::config()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(handle::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(constants::Data { guild_service })
            })
        })
        .build()
}
