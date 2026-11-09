use std::time::Instant;

use moelle_sample::Choice;
use poise::serenity_prelude::{Mentionable, User};
use poise::{CreateReply, serenity_prelude::CreateEmbed};
use seren::TimestampConvert;

use crate::{Context, icode, is_bot, reply};

#[poise::command(slash_command)]
pub(crate) async fn greet(ctx: Context<'_>) -> anyhow::Result<()> {
    crate::info!("{}: {}", stringify!(self::greet), ctx.author().name);
    is_bot!(ctx);

    const GREETINGS: [&str; 3] = ["Hello", "こんにちは", "謝謝"];

    let now = chrono::Utc::now();
    let mut rng = moelle_a::Sfc64::from(now.timestamp() as u64);
    let reply = reply!("{}", rng.choice(&GREETINGS).unwrap_or(&"挨拶しないマン"));
    ctx.send(reply).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub(crate) async fn ping(ctx: Context<'_>) -> anyhow::Result<()> {
    crate::info!("{}: {}", stringify!(self::ping), ctx.author().name);
    is_bot!(ctx);

    let member = ctx
        .author_member()
        .await
        .ok_or(anyhow::anyhow!("Failed to fetch member"))?;

    let start = Instant::now();

    let message = ctx
        .send(reply!("{} pong!🏓", member.mention()).ephemeral(true))
        .await?;

    let elapsed = start.elapsed() * 2;

    let reply = reply!("{} pong!🏓 ({} ms)", member.mention(), elapsed.as_millis());
    message.edit(ctx, reply).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub(crate) async fn info(ctx: Context<'_>, user: Option<User>) -> anyhow::Result<()> {
    crate::info!("{}: {}", stringify!(self::info), ctx.author().name);
    is_bot!(ctx);

    ctx.defer().await?;

    let user = user.as_ref().unwrap_or(ctx.author());

    let created_at = user.created_at().naive_local();
    let a = format!(
        "{} ({})",
        created_at.to_short_date_time(),
        created_at.to_relative()
    );

    let embed = CreateEmbed::new()
        .title("👤ユーザー情報👤")
        .field("ユーザー名", icode!("{}", user.name), true)
        .field("ユーザーID", icode!("{}", user.id.to_string()), true)
        .field("アカウント作成日", a, false)
        .image(user.avatar_url().unwrap_or(user.default_avatar_url()));

    let reply = CreateReply::default().embed(embed).ephemeral(true);
    ctx.send(reply).await?;

    Ok(())
}
