use crate::app_settings::get_settings_sync;
use crate::deezer::get_dz_client;
use crate::handlers::index_artist_top_tracks;
use crate::search::get_meilisearch;
use crate::tools::MusicError;
use crate::{db::get_mongo, models::Artist};
use actix::clock::sleep;
use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use chrono::Duration;
use itertools::Itertools;
use log::info;

pub struct ArtistScraperActor;

impl ArtistScraperActor {
    async fn scrape_artist(artist: Artist, interval: Duration, cooldown: std::time::Duration) {
        if artist.should_update(interval) {
            info!("Scraping artist: {}", artist.name);
            let mut compl_artist = artist.clone();
            let db = get_mongo(None).await;
            let search = get_meilisearch(None).await;
            let dz = get_dz_client();

            let related = dz.get_related_artists(&artist.id).await.unwrap();
            let top_tracks = dz.get_artist_top_tracks(&artist.id).await.unwrap();

            let rel_artists: Vec<Artist> = related
                .data
                .clone()
                .into_iter()
                .map(Artist::from)
                .unique_by(|x| x.id)
                .collect_vec();
            let _ = db.bulk_insert_artists(&rel_artists).await;
            let _ = search.index_artists(rel_artists.clone()).await;
            let _ = db
                .set_related_artists(
                    &artist.id,
                    rel_artists.clone().into_iter().map(|x| x.id).collect_vec(),
                )
                .await;

            let tracks = index_artist_top_tracks(&top_tracks, &artist.id)
                .await
                .unwrap();
            db.set_top_tracks(
                &artist.id,
                &tracks.clone().into_iter().map(|x| x.id).collect_vec(),
            )
            .await
            .unwrap();
            compl_artist.top_tracks = Some(tracks.into_iter().map(|x| x.id).collect_vec());
            compl_artist.related_artists =
                Some(rel_artists.into_iter().map(|x| x.id).collect_vec());
            sleep(cooldown).await;
        }
    }
}

impl Actor for ArtistScraperActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Started Artist scraper");
        let config = get_settings_sync();
        ctx.run_interval(config.artist_periodic_scrape_check_interval, |act, ctx| {
            info!("Starting periodic artist scraping");
            let fut = Box::pin(async move {
                let db = get_mongo(None).await;
                let config = get_settings_sync();
                let artists = db
                    .get_outdated_artist(config.get_artist_periodic_scrape_update_interval())
                    .await
                    .unwrap();
                info!(
                    "Periodic artist scraping: {} artists to scrape",
                    artists.len()
                );
                for artist in artists {
                    ArtistScraperActor::scrape_artist(
                        artist,
                        config.get_artist_scrape_update_interval(),
                        config.artist_scrape_cooldown,
                    )
                    .await;
                }
            });

            let actor_fut = fut.into_actor(act);
            ctx.wait(actor_fut)
        });
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), MusicError>")]
pub struct ArtistScraperMessage(pub Artist);

impl Handler<ArtistScraperMessage> for ArtistScraperActor {
    type Result = Result<(), MusicError>;

    fn handle(&mut self, msg: ArtistScraperMessage, ctx: &mut Self::Context) -> Self::Result {
        let ArtistScraperMessage(artist) = msg;

        let fut = Box::pin(async move {
            let config = get_settings_sync();
            ArtistScraperActor::scrape_artist(
                artist,
                config.get_artist_scrape_update_interval(),
                config.artist_scrape_cooldown,
            )
            .await
        });

        let actor_fut = fut.into_actor(self);
        ctx.wait(actor_fut);
        Ok(())
    }
}
