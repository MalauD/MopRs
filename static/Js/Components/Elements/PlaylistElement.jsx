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
            _id: PropTypes.number,
            title: PropTypes.string.isRequired,
            artist_name: PropTypes.string.isRequired,
            image_url: PropTypes.string.isRequired,
        }).isRequired,
    };

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
    };

    ShowAddToNewPlaylistModal = () => {
        this.setState({
            ShowAddToNewPlaylistModal: true,
        });
    };

    CloseAddToPlaylistModal = () => {
        this.setState({
            ShowAddToPlaylistModal: false,
        });
    };

    CloseAddToNewPlaylistModal = () => {
        this.setState({
            ShowAddToNewPlaylistModal: false,
        });
    };

    render() {
        const { ShowAddToNewPlaylistModal, ShowAddToPlaylistModal } = this.state;
        const { Music, UseDragHandle } = this.props;
        const { title, artist_name, image_url } = Music;

        return (
            <>
                {ShowAddToPlaylistModal && (
                    <AddToPlaylistModal Music={Music} OnClose={this.CloseAddToPlaylistModal} />
                )}
                {ShowAddToNewPlaylistModal && (
                    <PlaylistCreateModal
                        MusicsId={[Music._id]}
                        OnClose={this.CloseAddToNewPlaylistModal}
                    />
                )}

                <MusicItemRow
                    ImageDz={image_url}
                    Title={title}
                    Artist={artist_name}
                    onClick={this.onPlaylistClick}
                    isAvailable
                    UseDragHandle={UseDragHandle}
                >
                    <Dropdown.Item onClick={this.onPlaylistClick}>Play</Dropdown.Item>
                    <Dropdown.Divider />
                    <Dropdown.Item onClick={this.ShowAddToPlaylistModal}>
                        Add to playlist
                    </Dropdown.Item>
                    <Dropdown.Item onClick={this.ShowAddToNewPlaylistModal}>
                        Add to a new playlist
                    </Dropdown.Item>
                </MusicItemRow>
            </>
        );
    }
}

export default PlaylistElement;
