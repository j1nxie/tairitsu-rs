use poise::serenity_prelude::UserId;

use crate::{Context, Error};

fn get_bot_id(ctx: Context<'_>) -> UserId {
    ctx.cache().current_user().id
}

#[poise::command(slash_command, prefix_command)]
pub async fn invite(ctx: Context<'_>) -> Result<(), Error> {
    if let Ok(invite_link) = std::env::var("INVITE_LINK") {
        ctx.say(format!("invite me at: {}", invite_link)).await?;
    } else {
        let bot_id = get_bot_id(ctx);
        ctx.say(format!("invite me at: https://discord.com/oauth2/authorize?client_id={}&permissions=414464724032&scope=bot+applications.commands", bot_id)).await?;
    }
    Ok(())
}
