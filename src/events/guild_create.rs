use crate::constants;
use poise::serenity_prelude::{self as serenity, model::guild};
pub async fn guild_create(data: &constants::Data, guild: guild::Guild) {
    if data
        .guild_service
        .create_guild(guild.id.to_string())
        .await
        .is_err()
    {
        println!("insert err");
    }
}
