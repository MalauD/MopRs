use crate::handlers::*;
use actix_web::web;

pub fn config_music(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/search/music/{search_req}", web::get().to(search_music))
            .route("/search/album/{search_req}", web::get().to(search_album))
            .route("/search/artist/{search_req}", web::get().to(search_artist))
            .route(
                "/search/playlist/{search_req}",
                web::get().to(search_playlist),
            )
            .route("/trending/musics", web::get().to(trending_musics))
            .route("/related/musics", web::post().to(get_related_musics))
            .route("/music/{id}", web::get().to(get_music))
            .route("/music/{id}/audio", web::get().to(get_music_audio))
            .route("/music/{id}/audio_tagged", web::get().to(get_music_tagged))
            .route("/music/{id}/like", web::get().to(like_music))
            .route("/album/{id}", web::get().to(get_album))
            .route("/artist/{id}", web::get().to(get_artist))
            .route("/playlist/create", web::post().to(create_playlist))
            .route(
                "/playlist/create/deezer",
                web::post().to(create_playlist_deezer),
            )
            .route("/playlist/{id}", web::get().to(get_playlist))
            .route("/playlist/{id}", web::delete().to(delete_playlist))
            .route("/playlist/{id}/musics", web::post().to(add_music_playlist))
            .route(
                "/playlist/{id}/musics",
                web::delete().to(remove_music_playlist),
            )
            .route(
                "/playlist/{id}/musics/edit",
                web::post().to(edit_music_playlist),
            )
            .route("/playlist/{id}/edit", web::post().to(edit_playlist))
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/logout", web::post().to(logout))
            .route("/me", web::get().to(get_account))
            .route("/me/likes/musics", web::get().to(get_liked))
            .route("/me/history/musics", web::get().to(get_viewed))
            .route("/me/currentplaylist", web::get().to(get_current_playlist))
            .route(
                "/me/currentplaylist/musics",
                web::post().to(set_current_playlist_musics),
            )
            .route(
                "/me/currentplaylist/playing",
                web::post().to(set_current_playlist_playing),
            )
            .route("/me/suggestions", web::get().to(get_suggestions))
            .route("/user/{id}/playlists", web::get().to(get_user_playlists)),
    );
}
