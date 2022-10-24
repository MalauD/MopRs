use crate::handlers::*;
use actix_web::web;

pub fn config_music(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/Music")
            .route(
                "/Search/Music/Name/{search_req}",
                web::get().to(search_music),
            )
            .route(
                "/Search/Album/Name/{search_req}",
                web::get().to(search_album),
            )
            .route(
                "/Search/Artist/Name/{search_req}",
                web::get().to(search_artist),
            )
            .route(
                "/Search/Playlist/Name/{search_req}",
                web::get().to(search_playlist),
            )
            .route("/Trending/Musics", web::get().to(trending_musics))
            .route("/Album/id/{id}", web::get().to(get_album))
            .route("/Artist/id/{id}", web::get().to(get_artist))
            .route("/Playlist/Create", web::post().to(create_playlist))
            .route(
                "/Playlist/Create/Deezer",
                web::post().to(create_playlist_deezer),
            )
            .route("/Playlist/id/{id}", web::get().to(get_playlist))
            .route("/Playlist/id/{id}", web::delete().to(delete_playlist))
            .route("/Playlist/id/{id}/Add", web::post().to(add_music_playlist))
            .route(
                "/Playlist/id/{id}/Edit",
                web::post().to(edit_music_playlist),
            )
            .route(
                "/Playlist/id/{id}/Remove",
                web::delete().to(remove_music_playlist),
            )
            .route("/cdn/{id}", web::get().to(get_music))
            .route("/Like/Music/{id}", web::get().to(like_music))
            .route("/Related", web::post().to(get_related_musics)),
    );
}
