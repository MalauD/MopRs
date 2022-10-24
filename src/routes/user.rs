use crate::handlers::*;
use actix_web::web;

pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/User")
            .route("/Login", web::post().to(login))
            .route("/Register", web::post().to(register))
            .route("/Logout", web::post().to(logout))
            .route("/Me", web::get().to(get_account))
            .route("/LikedMusics", web::get().to(get_liked))
            .route("/ViewedMusics", web::get().to(get_viewed))
            .route("/CurrentPlaylist", web::get().to(get_current_playlist))
            .route(
                "/CurrentPlaylist/Musics",
                web::post().to(set_current_playlist_musics),
            )
            .route(
                "/CurrentPlaylist/Playing",
                web::post().to(set_current_playlist_playing),
            )
            .route("/{id}/Playlists", web::get().to(get_user_playlists)),
    );
}
