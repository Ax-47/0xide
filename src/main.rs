use dotenv::dotenv;
use oxide::create;
use poise::serenity_prelude as serenity;
/// Displays your or another user's account creation date

#[tokio::main]
async fn main() {
    dotenv().expect("missing .env file");
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();
    let framework = create::create();
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
