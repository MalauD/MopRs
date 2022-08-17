import { ADD_MY_ACCOUNT, LIKE_MUSIC } from '../Actions/Action';

const InitialState = {
    Account: undefined,
};

/* eslint default-param-last: ["off"] */

export default function UserAccountReducer(state = InitialState, action) {
    switch (action.type) {
        case ADD_MY_ACCOUNT:
            return {
                ...state,
                Account: action.Account,
            };
        case LIKE_MUSIC:
            return {
                ...state,
                Account: {
                    ...state.Account,
                    liked_musics:
                        state.Account.liked_musics.indexOf(action.MusicId) === -1
                            ? [...state.Account.liked_musics, action.MusicId]
                            : state.Account.liked_musics.filter((id) => id !== action.MusicId),
                },
            };
        default:
            return state;
    }
}
