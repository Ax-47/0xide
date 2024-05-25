use scylla::macros::FromRow;
use scylla::macros::{FromUserType, SerializeCql};
#[derive(Debug, Clone, FromUserType, SerializeCql)]
pub struct TextPoint {
    pub text: String,
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone, FromUserType, SerializeCql)]
pub struct Image {
    pub path: String,
    pub text: Vec<TextPoint>,
}

#[derive(Debug, Clone, FromUserType, SerializeCql)]
pub struct Embed {
    pub title: String,
    pub description: String,
    pub colour: i64,
    pub author: String,
    pub thumbnail: String,
    pub image: Image,
}

#[derive(Debug, Clone, FromRow)]
pub struct Guild {
    pub id: String,
    pub join_channel: String,
    pub leave_channel: String,
    pub join_embed: Embed,
    pub leave_embed: Embed,
}
