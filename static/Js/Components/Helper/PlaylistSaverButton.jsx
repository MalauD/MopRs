import React from 'react';
import { withRouter } from 'react-router-dom';
import PropTypes from 'prop-types';
import {
	Modal, Button, Form,
} from 'react-bootstrap';
import Axios from 'axios';
import ButtonIcon from './ButtonIcon';

class PlaylistSaverButton extends React.Component {
	static propTypes = {
		MusicsId: PropTypes.arrayOf(PropTypes.string).isRequired,
		history: PropTypes.shape({ push: PropTypes.func }).isRequired,
	};

	constructor(props) {
		super(props);
		this.state = {
			ShowModal: false,
			Name: undefined,
			IsPublic: true,
		};
	}

	handleSubmit = () => {
		const { MusicsId, history } = this.props;
		const { Name, IsPublic } = this.state;

		Axios.post('/Music/Playlist/Create/', { Name, IsPublic, MusicsId })
			.then((res) => {
				history.push(`/Playlist/${res.data.CreatedPlaylistId}`);
			})
			.catch((err) => console.log(err));
	};

	closeModal = () => {
		this.setState({ ShowModal: false });
	};

	openModal = () => {
		this.setState({ ShowModal: true });
	};

	onNameChange = (event) => {
		this.setState({ Name: event.target.value });
	}

	handleIsPublicChange = () => {
		this.setState((prevState) => ({ IsPublic: !prevState.IsPublic }));
	}

	render() {
		const { ShowModal, IsPublic, Name } = this.state;

		return (
			<>
				<ButtonIcon
					dataEva={"save"}
					buttonClass="float-right d-none d-lg-block"
					onClick={this.openModal} 
					evaOptions={{fill: "#d6d6d6ff", width: '30px', height: '30px'}} 
				/>

				<Modal show={ShowModal} onHide={this.closeModal}>
					<Modal.Header closeButton>
						<Modal.Title>Save playlist</Modal.Title>
					</Modal.Header>
					<Modal.Body>
						<Form>
							<Form.Group controlId="Name">
								<Form.Label>Name</Form.Label>
								<Form.Control value={Name} onChange={this.onNameChange} placeholder="Enter a playlist name" />
							</Form.Group>
							<Form.Group controlId="IsPublic">
								<Form.Check type="checkbox" label="Public" checked={IsPublic} onChange={this.handleIsPublicChange} />
							</Form.Group>
						</Form>
					</Modal.Body>
					<Modal.Footer>
						<Button variant="primary" onClick={this.handleSubmit}>Save</Button>
						<Button variant="outline-primary" onClick={this.closeModal}>Cancel</Button>
					</Modal.Footer>
				</Modal>
			</>
		);
	}
}

export default withRouter(PlaylistSaverButton);
