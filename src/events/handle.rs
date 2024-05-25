use crate::constants;
use poise::serenity_prelude as serenity;

use super::guild_create::guild_create;
use super::on_ready::on_ready;
pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, constants::Data, constants::Error>,
    data: &constants::Data,
) -> Result<(), constants::Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            on_ready(data_about_bot.to_owned()).await
        }
        serenity::FullEvent::GuildCreate { guild, .. } => {
            guild_create(data, guild.to_owned()).await
        }
        _ => {}
    }
    Ok(())
}
