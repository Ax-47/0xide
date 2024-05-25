use crate::{constants, events::on_ready};
use poise::serenity_prelude as serenity;
pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, constants::Data, constants::Error>,
    _data: &constants::Data,
) -> Result<(), constants::Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            on_ready::on_ready(data_about_bot.to_owned()).await;
        }
        _ => {}
    }
    Ok(())
}
