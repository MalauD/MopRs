import { handleCurrentPlaylistMusicsChange, handleCurrentPlaylistPlayingMusicChange } from './CurrentPlaylist';

const onStoreDispatch = (store) => {
	const state = store.getState();
	handleCurrentPlaylistMusicsChange(state);
	handleCurrentPlaylistPlayingMusicChange(state);
};

export default onStoreDispatch;
