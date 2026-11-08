use poise::FrameworkOptions;
use serenity::all::{ActivityData, GatewayIntents, GuildId, OnlineStatus};

#[bitflags2::flags(u16)]
pub enum CommonCommands {
    Ping,
    Info,
    Greet,
}

#[derive(Debug, Clone)]
pub struct PresenceOptions {
    pub activity: Option<ActivityData>,
    pub status: OnlineStatus,
}

#[derive(Debug, Clone, Copy)]
pub enum CommandRegistration {
    Global,
    Guild(GuildId),
    None,
}

#[derive(Debug, Clone)]
pub struct SetupOptions {
    pub presence: Option<PresenceOptions>,
    pub registration: CommandRegistration,
    pub log_ready: bool,
}

pub struct FrameworkConfig {
    pub options: FrameworkOptions<crate::Data, anyhow::Error>,
    pub setup: SetupOptions,
    pub common_commands: Option<CommonCommands>,
}

pub struct StartOptions<'a> {
    pub token_key: Option<&'a str>,
    pub intents: GatewayIntents,
    pub framework: FrameworkConfig,
}
