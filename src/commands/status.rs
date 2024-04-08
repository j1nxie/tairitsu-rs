use std::time::UNIX_EPOCH;

use crate::{
    commands::{get_bot_avatar, get_invite_link},
    constants::{version::get_version, POISE_VERSION, STARTUP_TIME},
    models::prelude::Users,
    Context, Error,
};
use poise::serenity_prelude as serenity;
use sea_orm::{EntityTrait, PaginatorTrait, QuerySelect};

/// get the bot's status
#[poise::command(prefix_command, slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    let users = Users::find().limit(None).count(&ctx.data().db).await?;

    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
        .field(
            "about the bot",
            format!(
                "[tairitsu](https://github.com/j1nxie/tairitsu-rs) is a Discord bot for Arcaea, written by [j1nxie](https://github.com/j1nxie), using the [poise](https://github.com/serenity-rs/poise) framework. invite me [here]({})!",
                get_invite_link(ctx)
            ),
            false
        )
        .field("version", get_version(), true)
        .field("users", users.to_string(), true)
        .field("servers", ctx.cache().guilds().len().to_string(), true)
        .field("rust", format!("[{0}](https://releases.rs/docs/{0})", rustc_version_runtime::version().to_string()), true)
        .field("poise", format!("[{0}](https://docs.rs/crate/poise/{0})", POISE_VERSION), true)
        .field("uptime", format!("<t:{}:R>", STARTUP_TIME.duration_since(UNIX_EPOCH).unwrap().as_secs()), true)
        .thumbnail(get_bot_avatar(ctx))
    )).await?;

    Ok(())
}
