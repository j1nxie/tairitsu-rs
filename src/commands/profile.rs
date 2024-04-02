use poise::serenity_prelude::{self as serenity, Color};
use reqwest::Method;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    constants::API_URL,
    models::{
        arcaea::{ClearStatsResponse, UserDataResponse},
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
                .header("Cookie", &token)
                .send()
                .await?;

            let clear_stats_response = ctx
                .data()
                .client
                .request(
                    Method::GET,
                    // defaults to FTR/ETR for now but i will look into the settings
                    format!("{}/clear_statistic?difficulty=2", API_URL),
                )
                .header("Cookie", &token)
                .send()
                .await?;

            let mut body = serde_json::from_str::<UserDataResponse>(&response.text().await?)?
                .value
                .unwrap();

            let clear_stats =
                serde_json::from_str::<ClearStatsResponse>(&clear_stats_response.text().await?)?
                    .value
                    .unwrap();

            // assuming that arcaea friend codes will forever be in the format of `xxx xxx xxx`.
            body.user_code.insert(3, ' ');
            body.user_code.insert(7, ' ');

            message
                .edit(
                    ctx,
                    // TODO: add in character icon
                    poise::CreateReply::default().content("").embed(
                        serenity::CreateEmbed::default()
                            .title(body.display_name)
                            .description(format!(
                                "- **Friend code**: {}\n- **Potential**: {} {}\n- **Joined**: <t:{}:f>\n- **Clear stats (FTR | ETR)**:\n  - {}/{7} clears\n  - {}/{7} full recalls\n  - {}/{7} pure memories",
                                body.user_code,
                                body.rating as f64 / 100.0,
                                match body.rating {
                                    1300.. => ":star: :star: :star:",
                                    1250..=1299 => ":star: :star:",
                                    1200..=1249 => ":star:",
                                    _ => "",
                                },
                                body.join_date / 1000,
                                clear_stats.clear,
                                clear_stats.full_recall,
                                clear_stats.pure_memory,
                                clear_stats.song_owned_count,
                            ))
                            .color(match body.rating {
                                1300.. => Color::from_rgb(178, 34, 34),
                                1250..=1299 => Color::from_rgb(139, 0, 139),
                                1200..=1249 => Color::from_rgb(220, 20, 60),
                                1100..=1199 => Color::from_rgb(139, 0, 0),
                                1000..=1099 => Color::from_rgb(128, 0, 128),
                                700..=999 => Color::from_rgb(75, 0, 130),
                                350..=699 => Color::from_rgb(0, 100, 0),
                                0..=349 => Color::from_rgb(25, 25, 112),
                                // just in case you can somehow hide your rating on site
                                // maybe i'll make that a setting
                                _ => Color::LIGHT_GREY,
                            }),
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
