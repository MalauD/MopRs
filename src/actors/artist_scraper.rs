use crate::app_settings::get_settings;
use crate::deezer::get_dz_client;
use crate::handlers::index_artist_top_tracks;
use crate::tools::MusicError;
use crate::{db::get_mongo, models::Artist};
use actix::clock::sleep;
use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use itertools::Itertools;
use log::info;

pub struct ArtistScraperActor;

impl Actor for ArtistScraperActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Started Artist scraper");
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
            let config = get_settings(None).await;
            if artist.should_update(config.get_artist_scrape_update_interval()) {
                info!("Scraping artist: {}", artist.name);
                let mut compl_artist = artist.clone();
                let db = get_mongo(None).await;
                let dz = get_dz_client(None).await.read().await;

                let related = dz.get_related_artists(&artist.id).await.unwrap();
                let top_tracks = dz.get_artist_top_tracks(&artist.id).await.unwrap();

                let rel_artists: Vec<Artist> = related
                    .data
                    .clone()
                    .into_iter()
                    .map(|x| Artist::from(x))
                    .unique_by(|x| x.id)
                    .collect_vec();
                let _ = db.bulk_insert_artists(&rel_artists).await;
                let _ = db
                    .set_related_artists(
                        &artist.id,
                        rel_artists.clone().into_iter().map(|x| x.id).collect_vec(),
                    )
                    .await;

                let tracks = index_artist_top_tracks(&top_tracks, &artist.id)
                    .await
                    .unwrap();
                let _ = db
                    .set_top_tracks(
                        &artist.id,
                        &tracks.clone().into_iter().map(|x| x.id).collect_vec(),
                    )
                    .await
                    .unwrap();
                compl_artist.top_tracks = Some(tracks.into_iter().map(|x| x.id).collect_vec());
                compl_artist.related_artists =
                    Some(rel_artists.into_iter().map(|x| x.id).collect_vec());
                sleep(config.artist_scrape_cooldown).await;
            }
        });

        let actor_fut = fut.into_actor(self);
        Ok(ctx.wait(actor_fut))
    }
}
