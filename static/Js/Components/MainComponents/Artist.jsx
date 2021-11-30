import React from 'react';
import PropTypes from 'prop-types';
import Axios from 'axios';
import AlbumGroup from './Groups/AlbumGroup';


class Artist extends React.Component {
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
			ArtistName: '',
			AlbumsId: [],
		};
	}

	componentDidMount = () => {
		const { match } = this.props;
		Axios.get(`/Music/Artist/id/${match.params.id}`).then((res) => {
			this.setState({
				ArtistName: res.data.name,
				AlbumsId: res.data.albums,
			});
		});
	}


	render() {
		const { ArtistName, AlbumsId } = this.state;

		return <AlbumGroup Albums={AlbumsId} DetailType={ArtistName} />;
	}
}

export default Artist;
