use crate::infrastructure::services::guild;
pub struct Data {
    pub guild_service: guild::GuildService,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
