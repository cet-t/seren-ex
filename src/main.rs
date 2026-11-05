mod commands;
mod macros;
mod toolkit;

use poise::builtins::register_globally;
use poise::{Framework, FrameworkOptions};
use serenity::all::{ActivityData, ClientBuilder, GatewayIntents, OnlineStatus, Ready};

#[derive(Debug, Clone, Copy, Default)]
pub struct Data;
pub type Context<'a> = poise::Context<'a, crate::Data, anyhow::Error>;
pub type Command = poise::Command<crate::Data, anyhow::Error>;

fn setup_presence(ctx: &serenity::all::Context) {
    ctx.set_presence(
        Some(ActivityData::listening("serenity template")),
        OnlineStatus::DoNotDisturb,
    );
}

fn setup<'a>(
    ctx: &'a serenity::all::Context,
    ready: &'a Ready,
    fw: &'a Framework<self::Data, anyhow::Error>,
) -> poise::BoxFuture<'a, Result<self::Data, anyhow::Error>> {
    info!("{} is connected!", ready.user.name);
    info!("servers: {}", ready.guilds.len());
    ready.guilds.iter().for_each(|g| info!("\t{}", g.id));

    setup_presence(ctx);

    Box::pin(async move {
        let commands = &fw.options().commands;
        register_globally(ctx, commands).await?;
        Ok(Default::default())
    })
}

fn build_fw() -> Framework<self::Data, anyhow::Error> {
    let builder = Framework::builder()
        .options(FrameworkOptions {
            commands: crate::commands::commands(),
            ..Default::default()
        })
        .setup(self::setup);

    builder.build()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = ClientBuilder::new(token!()?, GatewayIntents::all())
        .framework(build_fw())
        .await?;
    client.start().await?;

    Ok(())
}
