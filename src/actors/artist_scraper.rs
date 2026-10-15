use crate::handlers::index_artist_top_tracks;
use crate::tools::MusicError;
use crate::{
    app_settings::AppSettings, db::MongoClient, deezer::DeezerClient, models::Artist,
    search::MeilisearchClient,
};
use actix::clock::sleep;
use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use actix_web::web::Data;
use chrono::Duration;
use itertools::Itertools;
use log::{info, warn};

pub struct ArtistScraperActor {
    db: Data<MongoClient>,
    search: Data<MeilisearchClient>,
    deezer: Data<DeezerClient>,
    config: AppSettings,
}

impl ArtistScraperActor {
    pub fn new(
        db: Data<MongoClient>,
        search: Data<MeilisearchClient>,
        deezer: Data<DeezerClient>,
        config: AppSettings,
    ) -> Self {
        Self {
            db,
            search,
            deezer,
            config,
        }
    }

    async fn scrape_artist(
        db: &MongoClient,
        search: &MeilisearchClient,
        dz: &DeezerClient,
        artist: Artist,
        interval: Duration,
        cooldown: std::time::Duration,
    ) {
        if artist.should_update(interval) {
            info!("Scraping artist: {}", artist.name);

            let related = match dz.get_related_artists(&artist.id).await {
                Ok(related) => related,
                Err(error) => {
                    warn!(
                        "Failed to fetch related artists for {}: {}",
                        artist.id, error
                    );
                    return;
                }
            };
            let top_tracks = match dz.get_artist_top_tracks(&artist.id).await {
                Ok(top_tracks) => top_tracks,
                Err(error) => {
                    warn!("Failed to fetch top tracks for {}: {}", artist.id, error);
                    return;
                }
            };

            let rel_artists: Vec<Artist> = related
                .data
                .clone()
                .into_iter()
                .map(Artist::from)
                .unique_by(|x| x.id)
                .collect_vec();
            let _ = db.bulk_insert_artists(&rel_artists).await;
            let _ = search.index_artists(&rel_artists).await;
            let _ = db
                .set_related_artists(
                    &artist.id,
                    rel_artists.iter().map(|artist| artist.id).collect_vec(),
                )
                .await;

            let tracks = match index_artist_top_tracks(db, search, &top_tracks, &artist.id).await {
                Ok(tracks) => tracks,
                Err(error) => {
                    warn!("Failed to index top tracks for {}: {}", artist.id, error);
                    return;
                }
            };
            if let Err(error) = db
                .set_top_tracks(
                    &artist.id,
                    &tracks.iter().map(|track| track.id).collect_vec(),
                )
                .await
            {
                warn!("Failed to persist top tracks for {}: {}", artist.id, error);
                return;
            }
            sleep(cooldown).await;
        }
    }
}

impl Actor for ArtistScraperActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Started Artist scraper");
        let scrape_check_interval = self.config.artist_periodic_scrape_check_interval;
        ctx.run_interval(scrape_check_interval, |act, ctx| {
            info!("Starting periodic artist scraping");
            let db = act.db.clone();
            let search = act.search.clone();
            let deezer = act.deezer.clone();
            let config = act.config.clone();
            let fut = Box::pin(async move {
                let artists = match db
                    .get_outdated_artist(config.get_artist_periodic_scrape_update_interval())
                    .await
                {
                    Ok(artists) => artists,
                    Err(error) => {
                        warn!("Failed to list outdated artists: {}", error);
                        return;
                    }
                };
                info!(
                    "Periodic artist scraping: {} artists to scrape",
                    artists.len()
                );
                for artist in artists {
                    ArtistScraperActor::scrape_artist(
                        db.get_ref(),
                        search.get_ref(),
                        deezer.get_ref(),
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
        let db = self.db.clone();
        let search = self.search.clone();
        let deezer = self.deezer.clone();
        let config = self.config.clone();

        let fut = Box::pin(async move {
            ArtistScraperActor::scrape_artist(
                db.get_ref(),
                search.get_ref(),
                deezer.get_ref(),
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
