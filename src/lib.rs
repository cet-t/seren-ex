mod commands;
pub mod config;
pub mod macros;
pub mod toolkit;

#[doc(hidden)]
pub mod __private {
    pub use {anyhow, chrono, dotenvy, log, poise, serenity, tokio};
}

pub use config::{
    CommandRegistration, CommonCommands, FrameworkConfig, PresenceOptions, SetupOptions,
    StartOptions,
};

use poise::Framework;
use poise::builtins::{register_globally, register_in_guild};
use serenity::all::{ClientBuilder, Ready};

#[derive(Debug, Clone, Copy, Default)]
pub struct Data;
pub type Context<'a> = poise::Context<'a, crate::Data, anyhow::Error>;
pub type Command = poise::Command<crate::Data, anyhow::Error>;

pub fn setup_presence(ctx: &serenity::all::Context, options: PresenceOptions) {
    ctx.set_presence(options.activity, options.status);
}

pub fn setup<'a>(
    ctx: &'a serenity::all::Context,
    ready: &'a Ready,
    fw: &'a Framework<self::Data, anyhow::Error>,
    options: SetupOptions,
) -> poise::BoxFuture<'a, Result<self::Data, anyhow::Error>> {
    if options.log_ready {
        info!("{} is connected!", ready.user.name);
        info!("servers: {}", ready.guilds.len());
        ready.guilds.iter().for_each(|g| info!("\t{}", g.id));
    }

    if let Some(presence) = options.presence {
        setup_presence(ctx, presence);
    }

    Box::pin(async move {
        let commands = &fw.options().commands;
        match options.registration {
            CommandRegistration::Global => register_globally(ctx, commands).await?,
            CommandRegistration::Guild(guild) => register_in_guild(ctx, commands, guild).await?,
            CommandRegistration::None => {}
        }
        Ok(Default::default())
    })
}

pub fn build_fw(mut config: FrameworkConfig) -> Framework<self::Data, anyhow::Error> {
    if let Some(flags) = config.common_commands {
        config.options.commands.extend(commands::commands(flags));
    }
    let initialize_owners = config.options.initialize_owners;
    Framework::builder()
        .options(config.options)
        .initialize_owners(initialize_owners)
        .setup(move |ctx, ready, fw| setup(ctx, ready, fw, config.setup))
        .build()
}

pub fn client_builder(options: StartOptions<'_>) -> anyhow::Result<ClientBuilder> {
    let token = match options.token_key {
        Some(key) => token!(key)?,
        None => token!()?,
    };
    Ok(ClientBuilder::new(token, options.intents).framework(build_fw(options.framework)))
}

/// ```no_run
/// use seren_ex::{
///     CommandRegistration, CommonCommands, FrameworkConfig, SetupOptions, StartOptions,
/// };
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     seren_ex::start(StartOptions {
///         token_key: None,
///         intents: serenity::all::GatewayIntents::GUILDS,
///         framework: FrameworkConfig {
///             options: poise::FrameworkOptions::default(),
///             common_commands: Some(CommonCommands::Ping | CommonCommands::Info),
///             setup: SetupOptions {
///                 presence: None,
///                 registration: CommandRegistration::Global,
///                 log_ready: true,
///             },
///         },
///     }).await
/// }
/// ```
pub async fn start(options: StartOptions<'_>) -> anyhow::Result<()> {
    let mut client = client_builder(options)?.await?;
    client.start().await?;

    Ok(())
}
