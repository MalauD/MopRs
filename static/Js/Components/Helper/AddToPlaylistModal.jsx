import React from 'react';
import PropTypes from 'prop-types';
import { withRouter } from 'react-router-dom';
import {
	Modal, Button, Spinner, ListGroup,
} from 'react-bootstrap';
import Axios from 'axios';

class AddToPlaylistModal extends React.Component {
	static propTypes = {
		OnClose: PropTypes.func.isRequired,
		history: PropTypes.shape({ push: PropTypes.func }).isRequired,
		Music: PropTypes.shape({
			_id: PropTypes.string.isRequired,
			Title: PropTypes.string.isRequired,
		}).isRequired,
	};

	constructor(props) {
		super(props);
		this.state = {
			UserPlaylists: [],
			IsLoading: false,
			SelectedPlaylistId: '',
		};
	}

	closeModal = () => {
		const { OnClose } = this.props;
		OnClose();
	};

	componentDidMount = () => {
		this.setState({
			IsLoading: true,
		});
		Axios.get('/User/Playlists').then((res) => {
			this.setState({
				UserPlaylists: res.data.PlaylistsId,
				IsLoading: false,
			});
		});
	};

	OnPlaylistSelect = (SelectedPlaylistId) => {
		this.setState({
			SelectedPlaylistId,
		});
	};

	handleSubmit = () => {
		const { SelectedPlaylistId } = this.state;
		const { Music } = this.props;
		Axios.post(`/Music/Playlist/id/${SelectedPlaylistId}/Add/`, {
			MusicsId: [Music._id],
		}).then(() => {
			this.closeModal();
		});
	};

	render() {
		const { UserPlaylists, IsLoading, SelectedPlaylistId } = this.state;
		const { Music } = this.props;

		const PlaylistsSmallItem = UserPlaylists.map((PlaylistApiRes) => (
			<ListGroup.Item
				action
				className={`PlaylistItem${PlaylistApiRes._id === SelectedPlaylistId ? ' PlaylistItemSelected' : ''}`}
				key={PlaylistApiRes._id}
				onClick={() => this.OnPlaylistSelect(PlaylistApiRes._id)}
			>
				{PlaylistApiRes.Name}
			</ListGroup.Item>
		));

		return (
			<Modal show onHide={this.closeModal}>
				<Modal.Header closeButton>
					<Modal.Title>
						Add
						{' '}
						{Music.Title}
						{' '}
						to playlist
					</Modal.Title>
				</Modal.Header>
				<Modal.Body>{!IsLoading && <ListGroup className="PlaylistSelector">{PlaylistsSmallItem}</ListGroup>}</Modal.Body>
				<Modal.Footer>
					<Button variant="primary" onClick={this.handleSubmit}>
						{IsLoading && <Spinner as="span" animation="border" size="sm" role="status" aria-hidden="true" />}
						Add
					</Button>
					<Button variant="outline-primary" onClick={this.closeModal}>
						Cancel
					</Button>
				</Modal.Footer>
			</Modal>
		);
	}
}

export default withRouter(AddToPlaylistModal);
