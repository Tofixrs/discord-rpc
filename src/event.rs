use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "evt", content = "data")]
pub enum Event {
    Ready {
        v: i32,
        config: ReadyConfig,
        user: PartialUser,
    },
    Error {
        code: i32,
        message: String,
    },
}

///DISCORD WHY
#[derive(Debug, Deserialize)]
pub struct SocketError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ReadyConfig {
    pub cdn_host: String,
    pub api_endpoint: String,
    pub environment: String,
}

#[derive(Debug, Deserialize)]
pub struct PartialUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub avatar_decoration: Option<String>,
    pub premium_type: Option<u8>,
}
