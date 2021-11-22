import React from 'react';
import PropTypes from 'prop-types';
import { withRouter } from 'react-router-dom';
import {
	Modal, Button, Form, Spinner,
} from 'react-bootstrap';
import Axios from 'axios';

class PlaylistCreateModal extends React.Component {
	static propTypes = {
		OnClose: PropTypes.func.isRequired,
		MusicsId: PropTypes.arrayOf(PropTypes.string).isRequired,
		history: PropTypes.shape({ push: PropTypes.func }).isRequired,
	};

	constructor(props) {
		super(props);
		this.state = {
			Name: '',
			IsPublic: true,
			IsLoading: false,
		};
	}

	handleSubmit = () => {
		const { IsPublic, Name } = this.state;
		const { history, MusicsId } = this.props;
		this.setState({ IsLoading: true });

		Axios.post('/Music/Playlist/Create/', { Name, IsPublic, MusicsId })
			.then((res) => {
				history.push(`/Playlist/${res.data.CreatedPlaylistId}`);
			})
			.catch((err) => console.log(err));
	};

	closeModal = () => {
		const { OnClose } = this.props;
		OnClose();
	};

	handleIsPublicChange = () => {
		this.setState((prevState) => ({ IsPublic: !prevState.IsPublic }));
	};

	onNameChange = (event) => {
		this.setState({ Name: event.target.value });
	}

	render() {
		const {
			IsPublic, IsLoading, Name,
		} = this.state;

		return (
			<Modal show onHide={this.closeModal}>
				<Modal.Header closeButton>
					<Modal.Title>Create playlist</Modal.Title>
				</Modal.Header>
				<Modal.Body>
					<Form>
						<Form.Group controlId="Name">
							<Form.Label>Name</Form.Label>
							<Form.Control value={Name} onChange={this.onNameChange} placeholder="Enter a playlist name" />
						</Form.Group>
						<Form.Group controlId="IsPublic">
							<Form.Check type="checkbox" label="Public" checked={IsPublic} onClick={this.handleIsPublicChange} />
						</Form.Group>
					</Form>
				</Modal.Body>
				<Modal.Footer>
					<Button variant="primary" onClick={this.handleSubmit}>
						Create
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

export default withRouter(PlaylistCreateModal);
