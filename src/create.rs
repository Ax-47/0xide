use crate::{commands::slash::ping, constants, infrastructure::services::guild};
pub async fn create(
    uri: String,
) -> poise::Framework<
    constants::Data,
    Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>,
> {
    let guild_service = guild::GuildService::new(uri).await.unwrap();
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping::ping()],
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
