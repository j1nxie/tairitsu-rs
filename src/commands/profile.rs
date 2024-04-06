use poise::serenity_prelude::{self as serenity, Color};
use reqwest::Method;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set,
};

use crate::{
    commands::login_error,
    constants::API_URL,
    models::{
        arcaea::{ClearStatsResponse, UserDataResponse},
        prelude::*,
        users,
    },
    Context, Error,
};

#[poise::command(prefix_command, slash_command, aliases("arcaea", "me"))]
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
            login_error(ctx).await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn login(
    ctx: Context<'_>,
    #[description = "your Arcaea Online token"] token: Option<String>,
) -> Result<(), Error> {
    if ctx.guild_channel().await.is_some() {
        ctx.send(
            poise::CreateReply::default()
                .content("login instructions have been sent to your DMs. (please enable **Privacy Settings > Direct Messages** if you have not received it.)")
        ).await?;

        ctx.author()
            .direct_message(
                &ctx,
                serenity::CreateMessage::default().content(
                    "**how to log into Tairitsu**:
1. go to [Arcaea Online](https://arcaea.lowiro.com/) using a PC or laptop.
2. open your browser's DevTools (Ctrl+Shift+I or F12).
3. go to the Application tab (click the `>>` arrows if you don't see it.)
4. under Storage > Cookie, click on the `https://arcaea.lowiro.com` entry and look for the entry with the name `sid` in the table to the right.
5. copy `sid`'s value in the table and type in `a>login <value>` in Tairitsu's DMs.
6. enjoy.
                    ",
                ),
            )
            .await?;

        return Ok(());
    }

    match token {
        Some(token) => {
            let user = Users::find()
                .filter(users::Column::DiscordId.eq(ctx.author().id.to_string()))
                .one(&ctx.data().db)
                .await?;

            let token = format!("sid={}", token);

            match user {
                Some(user) => {
                    let mut user = user.into_active_model();
                    user.arcaea_token = Set(Some(token));

                    user.update(&ctx.data().db).await?;
                }

                None => {
                    let user = users::ActiveModel {
                        discord_id: Set(ctx.author().id.to_string()),
                        arcaea_token: Set(Some(token)),
                        ..Default::default()
                    };

                    user.insert(&ctx.data().db).await?;
                }
            }

            ctx.reply("you are now logged into the bot!").await?;
        }

        None => {
            ctx.author()
                .direct_message(
                    &ctx,
                    serenity::CreateMessage::default().content(
                        "**how to log into Tairitsu**:
1. go to [Arcaea Online](https://arcaea.lowiro.com/) using a PC or laptop.
2. open your browser's DevTools (Ctrl+Shift+I or F12).
3. go to the Application tab (click the `>>` arrows if you don't see it.)
4. under Storage > Cookie, click on the `https://arcaea.lowiro.com` entry and look for the entry with the name `sid` in the table to the right.
5. copy `sid`'s value in the table and type in `a>login <value>` in Tairitsu's DMs.
6. enjoy.
                        ",
                    ),
                )
                .await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn logout(ctx: Context<'_>) -> Result<(), Error> {
    let user = Users::find()
        .filter(users::Column::DiscordId.eq(ctx.author().id.to_string()))
        .one(&ctx.data().db)
        .await?;

    let token = user.clone().and_then(|x| x.arcaea_token);

    match token {
        Some(_) => {
            let message = ctx
                .send(poise::CreateReply::default().content("one second..."))
                .await?;

            let mut user: users::ActiveModel = user.unwrap().into();
            user.arcaea_token = ActiveValue::set(None);
            user.update(&ctx.data().db).await?;

            message
                .edit(
                    ctx,
                    poise::CreateReply::default().content("you are now logged out of the bot!"),
                )
                .await?;
        }
        None => {
            ctx.send(poise::CreateReply::default().content("you are not logged into the bot!"))
                .await?;
        }
    }

    Ok(())
}
