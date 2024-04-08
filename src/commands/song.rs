use poise::serenity_prelude as serenity;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use simsearch::SimSearch;

use crate::{
    models::{
        charts, jackets,
        prelude::{Charts, Jackets, Songs},
    },
    Context, Error,
};

#[poise::command(slash_command, prefix_command, aliases("info"))]
pub async fn song(
    ctx: Context<'_>,
    #[description = "the song to search"] query: String,
) -> Result<(), Error> {
    let message = ctx
        .send(poise::CreateReply::default().content("one second..."))
        .await?;

    let mut engine = SimSearch::new();

    for song in Songs::find().all(&ctx.data().db).await? {
        engine.insert(song.id, &song.title);
    }

    let results = engine.search(&query);

    let song = results.first();

    match song {
        Some(song) => {
            let song = Songs::find_by_id(*song).one(&ctx.data().db).await?.unwrap();
            let jacket = Jackets::find()
                .filter(jackets::Column::SongId.eq(&song.ingame_id))
                .one(&ctx.data().db)
                .await?;
            let charts = Charts::find()
                .filter(charts::Column::SongId.eq(&song.ingame_id))
                .all(&ctx.data().db)
                .await?;

            let mut chart_string = String::new();

            for chart in charts {
                chart_string =
                    chart_string + &format!(" • {} ({})", chart.difficulty, chart.constant);
            }

            message
                .edit(
                    ctx,
                    poise::CreateReply::default().content("").embed(
                        serenity::CreateEmbed::new()
                            .title(song.title)
                            .description(song.artist)
                            .field("pack", song.pack, false)
                            .field(
                                "release date",
                                song.release_date.format("%m/%d/%Y").to_string(),
                                true,
                            )
                            .field("bpm", song.bpm, true)
                            .field("version", song.version, true)
                            .field("difficulties", chart_string, false)
                            .image(match jacket {
                                Some(jacket) => jacket.jacket_url,
                                None => String::new(),
                            }),
                    ),
                )
                .await?;
        }

        None => {
            message
                .edit(
                    ctx,
                    poise::CreateReply::default().content("").embed(
                        serenity::CreateEmbed::new()
                            .title("no song found!")
                            .description(format!("no song was found for the query `{}`", &query)),
                    ),
                )
                .await?;
        }
    }
    Ok(())
}
