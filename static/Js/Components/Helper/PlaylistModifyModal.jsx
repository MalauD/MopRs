import React from 'react';
import PropTypes from 'prop-types';
import { Modal, Button, Form } from 'react-bootstrap';
import Axios from 'axios';
import ButtonIcon from './ButtonIcon';

export default class PlaylistModifyModal extends React.Component {
    static propTypes = {
        PlaylistId: PropTypes.string.isRequired,
        OldName: PropTypes.string.isRequired,
        OldIsPublic: PropTypes.bool.isRequired,
        OnPlaylistModify: PropTypes.func.isRequired,
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
        const { PlaylistId, OnPlaylistModify } = this.props;
        const { Name, IsPublic } = this.state;

        Axios.post(`/api/playlist/${PlaylistId}/edit`, { Name, IsPublic }).then(() => {
            this.closeModal();
            OnPlaylistModify({ Name, IsPublic });
        });
    };

    closeModal = () => {
        this.setState({ ShowModal: false });
    };

    openModal = () => {
        const { OldName, OldIsPublic } = this.props;

        this.setState({ ShowModal: true, Name: OldName, IsPublic: OldIsPublic });
    };

    onNameChange = (event) => {
        this.setState({ Name: event.target.value });
    };

    handleIsPublicChange = () => {
        this.setState((prevState) => ({ IsPublic: !prevState.IsPublic }));
    };

    render() {
        const { ShowModal, IsPublic, Name } = this.state;

        return (
            <>
                <ButtonIcon
                    dataEva="settings-outline"
                    onClick={this.openModal}
                    evaOptions={{ fill: '#d6d6d6ff', width: '30px', height: '30px' }}
                />

                <Modal show={ShowModal} onHide={this.closeModal}>
                    <Modal.Header closeButton>
                        <Modal.Title>Edit playlist</Modal.Title>
                    </Modal.Header>
                    <Modal.Body>
                        <Form>
                            <Form.Group controlId="Name">
                                <Form.Label>Name</Form.Label>
                                <Form.Control
                                    value={Name}
                                    onChange={this.onNameChange}
                                    placeholder="Enter a playlist name"
                                />
                            </Form.Group>
                            <Form.Group controlId="IsPublic">
                                <Form.Check
                                    type="checkbox"
                                    label="Public"
                                    checked={IsPublic}
                                    onChange={this.handleIsPublicChange}
                                />
                            </Form.Group>
                        </Form>
                    </Modal.Body>
                    <Modal.Footer>
                        <Button variant="primary" onClick={this.handleSubmit}>
                            Edit
                        </Button>
                        <Button variant="outline-primary" onClick={this.closeModal}>
                            Cancel
                        </Button>
                    </Modal.Footer>
                </Modal>
            </>
        );
    }
}
