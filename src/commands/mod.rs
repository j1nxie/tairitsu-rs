use poise::serenity_prelude::{self as serenity, Color};

use crate::{Context, Error};

pub mod help;
pub mod profile;
pub mod recent;
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
