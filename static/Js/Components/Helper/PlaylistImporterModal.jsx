import React from 'react';
import PropTypes from 'prop-types';
import { withRouter } from 'react-router-dom';
import {
	Modal, Button, Form, Spinner,
} from 'react-bootstrap';
import Axios from 'axios';

class PlaylistImporterModal extends React.Component {
	static propTypes = {
		OnClose: PropTypes.func.isRequired,
		history: PropTypes.shape({ push: PropTypes.func }).isRequired,
	};

	constructor(props) {
		super(props);
		this.state = {
			Name: '',
			PlaylistDeezerUrl: '',
			ParsedDeezerUrl: undefined,
			IsPublic: true,
			IsLoading: false,
		};
	}

	handleSubmit = () => {
		const { ParsedDeezerUrl, IsPublic, Name } = this.state;
		const { history } = this.props;
		this.setState({ IsLoading: true });

		Axios.post('/Music/Playlist/Create/Deezer', { Name, IsPublic, DeezerId: ParsedDeezerUrl })
			.then((res) => {
				this.setState({ IsLoading: false });
				this.closeModal();
				history.push(`/Playlist/${res.data.CreatedPlaylistId}`);
			})
			.catch((err) => {
				console.log(err);
				this.setState({ IsLoading: false });
			});
	};

	closeModal = () => {
		const { OnClose } = this.props;
		OnClose();
	};

	onPlaylistUrlChange = (event) => {
		const RegexPlaylistUrl = /(?<=\/playlist\/).*/;
		const Matches = event.target.value.match(RegexPlaylistUrl);
		let DeezerId;
		if (Matches) {
			if (Matches.length === 1) {
				const parsed = parseInt(Matches[0], 10);
				if (!isNaN(parsed)) DeezerId = parsed;
			}
		}

		this.setState({ PlaylistDeezerUrl: event.target.value, ParsedDeezerUrl: DeezerId });
	};

	handleIsPublicChange = () => {
		this.setState((prevState) => ({ IsPublic: !prevState.IsPublic }));
	};

	onNameChange = (event) => {
		this.setState({ Name: event.target.value });
	}

	render() {
		const {
			PlaylistDeezerUrl, IsPublic, ParsedDeezerUrl, IsLoading, Name,
		} = this.state;

		return (
			<Modal show onHide={this.closeModal}>
				<Modal.Header closeButton>
					<Modal.Title>Import Deezer Playlist</Modal.Title>
				</Modal.Header>
				<Modal.Body>
					<Form>
						<Form.Group controlId="Name">
							<Form.Label>Name</Form.Label>
							<Form.Control value={Name} onChange={this.onNameChange} placeholder="Enter a playlist name" />
						</Form.Group>
						<Form.Group controlId="PlaylistDzUrl">
							<Form.Label>Playlist Deezer Url</Form.Label>
							<Form.Control value={PlaylistDeezerUrl} onChange={this.onPlaylistUrlChange} placeholder="https://www.deezer.com/fr/playlist/53362031" />
						</Form.Group>
						<Form.Group controlId="IsPublic">
							<Form.Check type="checkbox" label="Public" checked={IsPublic} onClick={this.handleIsPublicChange} />
						</Form.Group>
					</Form>
				</Modal.Body>
				<Modal.Footer>
					<Button variant="primary" disabled={ParsedDeezerUrl === undefined} onClick={this.handleSubmit}>
						Import
						{IsLoading && (
							<Spinner
								as="span"
								animation="border"
								size="sm"
								role="status"
								aria-hidden="true"
							/>
						)}
					</Button>
					<Button variant="outline-primary" onClick={this.closeModal}>
						Cancel
					</Button>
				</Modal.Footer>
			</Modal>
		);
	}
}

export default withRouter(PlaylistImporterModal);
