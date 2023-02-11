import {
    CHANGE_PLAYING_MUSIC,
    ADD_MUSIC,
    REMOVE_MUSIC,
    CHANGE_PLAYING_ID,
    CLEAR_PLAYLIST,
    ADD_MULTIPLE_MUSICS,
    UPDATE_CURRENT_PLAYLIST,
    PLAY_NEXT,
} from '../Actions/Action';

const initialState = {
    Playlist: {
        PlayingId: -1,
        Musics: [],
    },
};

function CalculateNewIdAfterRemove(oldId, Index) {
    if (oldId === Index) {
        return Index - 1;
    }
    if (oldId > Index) {
        return oldId - 1;
    }
    return oldId;
}

/* eslint default-param-last: ["off"] */

export default function MusicPlayerReducer(state = initialState, action) {
    switch (action.type) {
        case CHANGE_PLAYING_MUSIC:
            return {
                ...state,
                Playlist: {
                    Musics: action.RemoveOthers
                        ? [action.NewMusic]
                        : [...state.Playlist.Musics, action.NewMusic],
                    PlayingId: action.RemoveOthers ? 0 : state.Playlist.Musics.length,
                },
            };
        case ADD_MUSIC:
            return {
                ...state,
                Playlist: {
                    Musics: [...state.Playlist.Musics, action.AddedMusic],
                    PlayingId: state.Playlist.PlayingId,
                },
            };
        case REMOVE_MUSIC:
            return {
                ...state,
                Playlist: {
                    Musics: [
                        ...state.Playlist.Musics.slice(0, action.Index),
                        ...state.Playlist.Musics.slice(action.Index + 1),
                    ],
                    PlayingId: CalculateNewIdAfterRemove(state.Playlist.PlayingId, action.Index),
                },
            };
        case PLAY_NEXT:
            return {
                ...state,
                Playlist: {
                    Musics: [
                        ...state.Playlist.Musics.slice(0, state.Playlist.PlayingId + 1),
                        action.NextMusic,
                        ...state.Playlist.Musics.slice(state.Playlist.PlayingId + 1),
                    ],
                    PlayingId: state.Playlist.PlayingId,
                },
            };
        case ADD_MULTIPLE_MUSICS:
            return {
                Playlist: {
                    Musics: [...state.Playlist.Musics, ...action.AddedMusics],
                    PlayingId: state.Playlist.PlayingId,
                },
            };
        case UPDATE_CURRENT_PLAYLIST:
            return {
                Playlist: {
                    Musics: action.UpdatedMusics,
                    PlayingId: action.UpdatedPlayingId,
                },
            };
        case CHANGE_PLAYING_ID:
            return {
                ...state,
                Playlist: {
                    PlayingId: action.PlaylistId,
                    Musics: state.Playlist.Musics,
                },
            };

        case CLEAR_PLAYLIST:
            return {
                ...state,
                Playlist: {
                    PlayingId: 0,
                    Musics: [],
                },
            };

        default:
            return state;
    }
}
