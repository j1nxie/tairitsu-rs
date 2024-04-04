use poise::serenity_prelude::{self as serenity, UserId};
use reqwest::Method;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use thousands::Separable;

use crate::{
    constants::API_URL,
    models::{
        arcaea::UserDataResponse,
        charts,
        prelude::{Charts, Songs, Users},
        songs, users,
    },
    Context, Error,
};

use super::login_error;

#[poise::command(slash_command, prefix_command, aliases("rs"))]
pub async fn recent(
    ctx: Context<'_>,
    #[description = "selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = Users::find()
        .filter(
            users::Column::DiscordId
                .eq(user.map_or(ctx.author().id.to_string(), |x| x.id.to_string())),
        )
        .one(&ctx.data().db)
        .await?
        .unwrap();

    let token = user.arcaea_token;

    match token {
        Some(token) => {
            let response = ctx
                .data()
                .client
                .request(Method::GET, API_URL)
                .header("Cookie", &token)
                .send()
                .await?;

            let body = serde_json::from_str::<UserDataResponse>(&response.text().await?)?
                .value
                .unwrap();

            let recent = body.recent_score.first();

            match recent {
                Some(recent) => {
                    let song = Songs::find()
                        .filter(songs::Column::IngameId.eq(&recent.song_id))
                        .one(&ctx.data().db)
                        .await?
                        .unwrap();

                    let diff_name = match recent.difficulty {
                        0 => "PST",
                        1 => "PRS",
                        2 => "FTR",
                        3 => "BYD",
                        4 => "ETR",
                        _ => unreachable!(),
                    };

                    let clear_type = match recent.clear_type {
                        // TODO: figure out the rest
                        0 => "track lost",
                        1 => "easy clear",
                        2 => "normal clear",
                        3 => "hard clear",
                        _ => todo!(),
                    };

                    let chart = Charts::find()
                        .filter(
                            charts::Column::SongId
                                .eq(&recent.song_id)
                                .and(charts::Column::Difficulty.eq(diff_name)),
                        )
                        .one(&ctx.data().db)
                        .await?
                        .unwrap();

                    let rank = match recent.score {
                        9_900_000.. => "EX+",
                        9_800_000..=9_899_999 => "EX",
                        9_500_000..=9_799_999 => "AA",
                        9_200_000..=9_499_999 => "A",
                        8_900_000..=9_199_999 => "B",
                        8_600_000..=8_899_999 => "C",
                        0..=8_599_999 => "D",
                    };

                    ctx.send(
                        poise::CreateReply::default().embed(
                            serenity::CreateEmbed::new()
                                .title(format!(
                                    "{} [{} {}]",
                                    &song.title, diff_name, chart.constant
                                ))
                                .field(
                                    "score",
                                    format!(
                                        "**{}**\n**{}** rank",
                                        recent.score.separate_with_commas(),
                                        rank,
                                    ),
                                    true,
                                )
                                .field(
                                    "judgements",
                                    format!(
                                        "**{}** (+{}) pures\n**{}** fars\n**{}** losts",
                                        recent.perfect_count,
                                        recent.shiny_perfect_count,
                                        recent.near_count,
                                        recent.miss_count
                                    ),
                                    true,
                                )
                                .field("clear type", clear_type, false)
                                .author(
                                    serenity::CreateEmbedAuthor::new(format!(
                                        "{} ({})",
                                        &body.display_name,
                                        body.rating as f64 / 100.0
                                    ))
                                    .icon_url(
                                        UserId::new(user.discord_id.parse::<u64>().unwrap())
                                            .to_user(ctx)
                                            .await?
                                            .avatar_url()
                                            .unwrap(),
                                    ),
                                )
                                .footer(serenity::CreateEmbedFooter::new(format!(
                                    "played at {}",
                                    chrono::DateTime::from_timestamp_millis(recent.time_played)
                                        .unwrap()
                                ))),
                        ),
                    )
                    .await?;
                }
                None => {
                    ctx.send(
                        poise::CreateReply::default().embed(
                            serenity::CreateEmbed::new()
                                .title("no recent score found!")
                                .description(format!(
                                    "no recent score was found for player {}.",
                                    body.display_name
                                )),
                        ),
                    )
                    .await?;
                }
            }
        }

        None => {
            login_error(ctx).await?;
        }
    }

    Ok(())
}
