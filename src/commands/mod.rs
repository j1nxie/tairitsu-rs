use poise::serenity_prelude::{self as serenity, Color, UserId};

use crate::{Context, Error};

pub mod help;
pub mod invite;
pub mod profile;
pub mod recent;
pub mod song;
pub mod status;

pub async fn login_error(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .color(Color::RED)
                .title("you're not logged in!")
                .description("send `a>login` to my DMs or use `/login` to start logging in."),
        ),
    )
    .await?;

    Ok(())
}

pub(crate) fn get_bot_id(ctx: Context<'_>) -> UserId {
    ctx.cache().current_user().id
}

pub(crate) fn get_bot_avatar(ctx: Context<'_>) -> String {
    ctx.cache().current_user().avatar_url().unwrap()
}

pub(crate) fn get_invite_link(ctx: Context<'_>) -> String {
    match std::env::var("INVITE_LINK") {
        Ok(invite_link) => invite_link,
        Err(_) => {
            let bot_id = get_bot_id(ctx);
            format!("https://discord.com/oauth2/authorize?client_id={}&permissions=414464724032&scope=bot+applications.commands", bot_id)
        }
    }
}
