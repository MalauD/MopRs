import React from 'react';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import MusicItemRow from '../Items/MusicItemRow';
import AddToPlaylistModal from '../Helper/AddToPlaylistModal';
import PlaylistCreateModal from '../Helper/PlaylistCreateModal';


class PlaylistElement extends React.Component {
	static propTypes = {
		ChangePlayingId: PropTypes.func.isRequired,
		UseDragHandle: PropTypes.bool.isRequired,
		Music: PropTypes.shape({
			_id: PropTypes.string,
			Title: PropTypes.string.isRequired,
			Artist: PropTypes.string.isRequired,
			AlbumId: PropTypes.shape({
				Image: PropTypes.string,
				ImagePathDeezer: PropTypes.string,
			}),
		}).isRequired,
	}

	constructor(props) {
		super(props);

		this.state = {
			ShowAddToPlaylistModal: false,
			ShowAddToNewPlaylistModal: false,
		};
	}

	onPlaylistClick = () => {
		const { ChangePlayingId } = this.props;
		ChangePlayingId();
	};

	ShowAddToPlaylistModal = () => {
		this.setState({
			ShowAddToPlaylistModal: true,
		});
	}

	ShowAddToNewPlaylistModal = () => {
		this.setState({
			ShowAddToNewPlaylistModal: true,
		});
	}

	CloseAddToPlaylistModal = () => {
		this.setState({
			ShowAddToPlaylistModal: false,
		});
	}

	CloseAddToNewPlaylistModal = () => {
		this.setState({
			ShowAddToNewPlaylistModal: false,
		});
	}

	render() {
		const { ShowAddToNewPlaylistModal, ShowAddToPlaylistModal } = this.state;
		const { Music, UseDragHandle } = this.props;
		const {
			AlbumId, Title, Artist,
		} = Music;

		return (
			<>
				{ShowAddToPlaylistModal
					&& <AddToPlaylistModal Music={Music} OnClose={this.CloseAddToPlaylistModal} />}
				{ShowAddToNewPlaylistModal
					&& (
						<PlaylistCreateModal
							MusicsId={[Music._id]}
							OnClose={this.CloseAddToNewPlaylistModal}
						/>
					)}
					
				<MusicItemRow
					Image={AlbumId.Image || undefined}
					ImageDz={AlbumId.ImagePathDeezer || undefined}
					Title={Title}
					Artist={Artist}
					onClick={this.onPlaylistClick}
					isAvailable
					UseDragHandle={UseDragHandle}
				>
					<Dropdown.Item onClick={this.onPlaylistClick}>Play</Dropdown.Item>
					<Dropdown.Divider />
					<Dropdown.Item onClick={this.ShowAddToPlaylistModal}>Add to playlist</Dropdown.Item>
					<Dropdown.Item onClick={this.ShowAddToNewPlaylistModal}>
						Add to a new playlist
					</Dropdown.Item>
				</MusicItemRow>
			</>
		);
	}
}

export default PlaylistElement;
