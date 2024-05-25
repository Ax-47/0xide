use crate::constants::*;
use poise::serenity_prelude::{self as serenity};
#[poise::command(slash_command, prefix_command, subcommands("set"))]
pub async fn config(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Selected Channel"]
    #[channel_types("Text")]
    channel: serenity::Channel,
) -> Result<(), Error> {
    let data = ctx.data();
    let _ = data
        .guild_service
        .update_join_channel(
            ctx.guild_id().unwrap_or_default().to_string(),
            channel.id().to_string(),
        )
        .await
        .expect("text");
    let response = format!("test config");
    ctx.say(response).await?;
    Ok(())
}
