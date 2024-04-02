use poise::serenity_prelude::{self as serenity, Color};
use reqwest::Method;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::{debug, info};

use crate::{
    constants::API_URL,
    models::{
        arcaea::{ArcaeaResponse, UserData},
        prelude::*,
        users,
    },
    Context, Error,
};

#[poise::command(prefix_command, slash_command)]
pub async fn profile(
    ctx: Context<'_>,
    #[description = "selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let token = Users::find()
        .filter(
            users::Column::DiscordId
                .eq(user.map_or(ctx.author().id.to_string(), |x| x.id.to_string())),
        )
        .one(&ctx.data().db)
        .await?
        .and_then(|x| x.arcaea_token);

    match token {
        Some(token) => {
            let message = ctx
                .send(poise::CreateReply::default().content("one second..."))
                .await?;
            let response = ctx
                .data()
                .client
                .request(Method::GET, API_URL)
                .header("Cookie", token)
                .send()
                .await?;

            let mut body: UserData =
                serde_json::from_str::<ArcaeaResponse>(&response.text().await?)?.value;

            // assuming that arcaea friend codes will forever be in the format of `xxx xxx xxx`.
            body.user_code.insert(3, ' ');
            body.user_code.insert(7, ' ');

            message
                .edit(
                    ctx,
                    poise::CreateReply::default().content("").embed(
                        serenity::CreateEmbed::default()
                            .title(body.display_name)
                            .description(format!(
                            "- **Friend code**: {}\n- **Potential**: {} {}\n- **Joined**: <t:{}:f>",
                            body.user_code,
                            body.rating as f64 / 100.0,
                            if body.rating >= 1300 {
                                ":star: :star: :star:"
                            } else if body.rating >= 1250 {
                                ":star: :star:"
                            } else if body.rating >= 1200 {
                                ":star:"
                            } else { "" },
                            body.join_date / 1000,
                        )).color(if body.rating >),
                    ),
                )
                .await?;
        }
        None => {
            ctx.send(
                poise::CreateReply::default().embed(
                    serenity::CreateEmbed::new()
                        .color(Color::RED)
                        .title("you're not logged in!")
                        .description(
                            "send `a>login` to my DMs or use `/login` to start logging in.",
                        ),
                ),
            )
            .await?;
        }
    }

    Ok(())
}
