import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import MusicGroup from './Groups/MusicGroup';
import { ALBUM_CONTEXT } from '../../Constants/MusicsConstants';

class Album extends React.Component {
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
			AlbumImage: undefined,
			AlbumImageDz: undefined,
			AlbumName: '',
		};
	}

	componentDidMount = () => {
		const { match } = this.props;

		Axios.get(`/Music/Album/id/${match.params.id}`).then((res) => {
			this.setState({
				Musics: res.data.MusicsId,
				AlbumImage: res.data.Image,
				AlbumImageDz: res.data.ImagePathDeezer,
				AlbumName: res.data.Name,
			});
		});
	};

	render() {
		const {
			Musics, AlbumName, AlbumImage, AlbumImageDz,
		} = this.state;

		if (Musics) {
			return (
				<MusicGroup
					CommonImage={AlbumImage}
					CommonImageDz={AlbumImageDz}
					Musics={Musics}
					DetailType={AlbumName}
					ContextType={ALBUM_CONTEXT}
				/>
			);
		}

		return <></>;
	}
}

export default Album;
