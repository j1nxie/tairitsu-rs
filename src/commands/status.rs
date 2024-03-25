use std::time::UNIX_EPOCH;

use crate::{constants::version::get_version, Context, Error};
use poise::serenity_prelude as serenity;

/// get the bot's status
#[poise::command(slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
        .field("about the bot", "[tairitsu](https://github.com/j1nxie/tairitsu-rs) is a Discord bot for Arcaea, written by [j1nxie](https://github.com/j1nxie).", false)
        .field("version", get_version(), false)
        .field("rust", format!("[{0}](https://releases.rs/docs/{0})", rustc_version_runtime::version().to_string()), true)
        .field("uptime", format!("<t:{}:R>", std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()), true)
    )).await?;

    Ok(())
}
