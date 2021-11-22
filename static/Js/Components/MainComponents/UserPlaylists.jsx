import React from 'react';
import PropTypes from 'prop-types';
import Axios from 'axios';
import UserPlaylistGroup from './Groups/UserPlaylistGroup';


class UserPlaylists extends React.Component {
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
			CreatorName: '',
			IsFetching: false,
			Playlists: [],
		};
	}

	componentDidMount = () => {
		const { match } = this.props;
		this.setState({
			IsFetching: true,
		});
		Axios.get(`/User/${match.params.id}/Playlists`).then((res) => {
			this.setState({
				CreatorName: res.data.Creator.username,
				Playlists: res.data.PlaylistsId,
				IsFetching: false,
			});
		});
	}


	render() {
		const { CreatorName, Playlists, IsFetching } = this.state;

		return <UserPlaylistGroup Playlists={Playlists} DetailType={`Playlists of ${CreatorName}`} IsFetching={IsFetching} />;
	}
}

export default UserPlaylists;
