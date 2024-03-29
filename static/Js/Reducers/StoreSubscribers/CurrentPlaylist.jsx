const { default: Axios } = require('axios');

let currentMusics;
const handleCurrentPlaylistMusicsChange = (state) => {
    const previousMusics = currentMusics;
    currentMusics = state.MusicPlayerReducer.Playlist.Musics;

    if (previousMusics !== currentMusics) {
        if (currentMusics.length !== 0) {
            Axios.post('/api/me/currentplaylist/musics', {
                CurrentPlaylist: currentMusics.map((m) => m._id),
            });
        }
    }
};

let currentPlayingMusics;
const handleCurrentPlaylistPlayingMusicChange = (state) => {
    const previousPlayingMusics = currentPlayingMusics;
    currentPlayingMusics = state.MusicPlayerReducer.Playlist.PlayingId;

    if (previousPlayingMusics !== currentPlayingMusics) {
        if (currentPlayingMusics !== -1)
            Axios.post('/api/me/currentplaylist/playing', {
                CurrentPlaylistPlaying: currentPlayingMusics,
            });
    }
};

export { handleCurrentPlaylistMusicsChange, handleCurrentPlaylistPlayingMusicChange };
