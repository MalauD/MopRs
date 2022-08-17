import React from 'react';
import PropTypes from 'prop-types';
import { withRouter } from 'react-router-dom';
import { Modal, Button, Spinner, ListGroup } from 'react-bootstrap';
import Axios from 'axios';
import { connect } from 'react-redux';

const mapStateToProps = (state) => ({
    Account: state.UserAccountReducer.Account,
});

class AddToPlaylistModalConnected extends React.Component {
    static propTypes = {
        OnClose: PropTypes.func.isRequired,
        history: PropTypes.shape({ push: PropTypes.func }).isRequired,
        Music: PropTypes.shape({
            _id: PropTypes.string.isRequired,
            Title: PropTypes.string.isRequired,
        }).isRequired,
        Account: PropTypes.shape().isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {
            UserPlaylists: [],
            IsLoading: false,
            SelectedPlaylistId: '',
        };
    }

    componentDidMount() {
        const { Account } = this.props;
        this.setState({
            IsLoading: true,
        });
        Axios.get(`/User/${Account._id}/Playlists?page=0&maxResults=100`).then((res) => {
            this.setState({
                UserPlaylists: res.data.Playlists,
                IsLoading: false,
            });
        });
    }

    OnPlaylistSelect = (SelectedPlaylistId) => {
        this.setState({
            SelectedPlaylistId,
        });
    };

    handleSubmit = () => {
        const { SelectedPlaylistId } = this.state;
        const { Music } = this.props;
        Axios.post(`/Music/Playlist/id/${SelectedPlaylistId}/Add`, {
            MusicsId: [Music._id],
        }).then(() => {
            this.closeModal();
        });
    };

    closeModal = () => {
        const { OnClose } = this.props;
        OnClose();
    };

    render() {
        const { UserPlaylists, IsLoading, SelectedPlaylistId } = this.state;
        const { Music } = this.props;

        const PlaylistsSmallItem = UserPlaylists.map((PlaylistApiRes) => (
            <ListGroup.Item
                action
                className={`PlaylistItem${
                    PlaylistApiRes._id === SelectedPlaylistId ? ' PlaylistItemSelected' : ''
                }`}
                key={PlaylistApiRes._id}
                onClick={() => this.OnPlaylistSelect(PlaylistApiRes._id)}
            >
                {PlaylistApiRes.name}
            </ListGroup.Item>
        ));

        return (
            <Modal show onHide={this.closeModal}>
                <Modal.Header closeButton>
                    <Modal.Title>Add {Music.Title} to playlist</Modal.Title>
                </Modal.Header>
                <Modal.Body>
                    {!IsLoading && (
                        <ListGroup className="PlaylistSelector">{PlaylistsSmallItem}</ListGroup>
                    )}
                </Modal.Body>
                <Modal.Footer>
                    <Button variant="primary" onClick={this.handleSubmit}>
                        {IsLoading && (
                            <Spinner
                                as="span"
                                animation="border"
                                size="sm"
                                role="status"
                                aria-hidden="true"
                            />
                        )}
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

const AddToPlaylistModal = connect(mapStateToProps)(AddToPlaylistModalConnected);

export default withRouter(AddToPlaylistModal);
