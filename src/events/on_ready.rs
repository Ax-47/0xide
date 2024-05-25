use poise::serenity_prelude as serenity;
pub async fn on_ready(data_about_bot: serenity::Ready) -> () {
    println!("Logged in as {}", data_about_bot.user.name);
}
