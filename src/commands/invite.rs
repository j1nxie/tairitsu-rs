use crate::{commands::get_invite_link, Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn invite(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("invite me at: {}", get_invite_link(ctx)))
        .await?;
    Ok(())
}
