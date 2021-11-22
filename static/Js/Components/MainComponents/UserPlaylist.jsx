import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import MusicGroup from './Groups/MusicGroup';
import { OWN_PLAYLIST_CONTEXT, PLAYLIST_CONTEXT } from '../../Constants/MusicsConstants';

class UserPlaylist extends React.Component {
	static propTypes = {
		match: PropTypes.shape({
			params: PropTypes.shape({
				id: PropTypes.string.isRequired,
			}).isRequired,
		}).isRequired,
	}

	constructor(props) {
		super(props);
		this.state = {
			Musics: undefined,
			PlaylistName: '',
			PlaylistId: '',
			CreatorName: '',
			OwnPlaylist: false,
		};
	}

	componentDidMount = () => {
		const { match } = this.props;

		Axios.get(`/Music/Playlist/id/${match.params.id}`).then((res) => {
			this.setState({
				Musics: res.data.MusicsId,
				PlaylistName: res.data.Name,
				PlaylistId: res.data._id,
				CreatorName: res.data.Creator.username,
				OwnPlaylist: res.data.HasControl,
			});
		});
	};

	render() {
		const {
			Musics,
			PlaylistName,
			CreatorName,
			OwnPlaylist,
			PlaylistId,
		} = this.state;

		if (Musics) {
			return (
				<MusicGroup
					Musics={Musics}
					DetailType={`${PlaylistName} by ${CreatorName}`}
					ContextType={OwnPlaylist ? OWN_PLAYLIST_CONTEXT : PLAYLIST_CONTEXT}
					ContextPlaylistId={PlaylistId}
				/>
			);
		}

		return <></>;
	}
}

export default UserPlaylist;
