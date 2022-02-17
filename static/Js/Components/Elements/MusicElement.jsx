import React from 'react';
import { connect } from 'react-redux';
import Axios from 'axios';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import { withRouter } from 'react-router-dom';
import MusicItemRow from '../Items/MusicItemRow';
import {
    ChangePlayingMusic as ChangePlayingMusicRedux,
    AddMusic as AddMusicRedux,
} from '../../Actions/Action';
import LikeButton from '../Helper/LikeButton';
import AddToPlaylistModal from '../Helper/AddToPlaylistModal';
import { OWN_PLAYLIST_CONTEXT } from '../../Constants/MusicsConstants';
import PlaylistCreateModal from '../Helper/PlaylistCreateModal';

const mapDispatchToProps = (dispatch) => ({
    ChangePlayingMusic: (Music) => {
        dispatch(ChangePlayingMusicRedux(Music));
    },
    AddMusic: (Music) => {
        dispatch(AddMusicRedux(Music));
    },
});

const mapStateToProps = (state) => {
    const { UserAccountReducer } = state;
    return { LikedMusics: UserAccountReducer.Account.liked_musics };
};

class MusicElementConnected extends React.Component {
    static propTypes = {
        history: PropTypes.shape({ go: PropTypes.func }).isRequired,
        ChangePlayingMusic: PropTypes.func.isRequired,
        AddMusic: PropTypes.func.isRequired,
        Music: PropTypes.shape({
            _id: PropTypes.number.isRequired,
            title: PropTypes.string.isRequired,
            artist_name: PropTypes.string.isRequired,
            file_path: PropTypes.string,
            image_url: PropTypes.string,
        }).isRequired,
        LikedMusics: PropTypes.arrayOf(PropTypes.number).isRequired,
        ContextType: PropTypes.string.isRequired,
        ContextPlaylistId: PropTypes.string,
    };

    static defaultProps = {
        ContextPlaylistId: undefined,
    };

    constructor(props) {
        super(props);
        this.state = {
            ShowAddToPlaylistModal: false,
            ShowAddToNewPlaylistModal: false,
        };
    }

    onClick = () => {
        const { ChangePlayingMusic, Music } = this.props;

        ChangePlayingMusic(Music);
    };

    HandleAdd = () => {
        const { AddMusic, Music } = this.props;

        AddMusic(Music);
    };

    HandleLike = () => {
        const { Music } = this.props;

        Axios.get(`/Music/Like/Music/${Music._id}`).then(() => {});
    };

    componentWillUnmount = () => {
        this.setState = () => {};
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

    HandleDeletePlaylistMusic = () => {
        const { ContextPlaylistId, Music, history } = this.props;
        Axios.delete(`/Music/Playlist/id/${ContextPlaylistId}/Remove`, {
            data: { MusicId: Music._id },
        }).then(() => history.go(0));
    };

    render() {
        const { ShowAddToPlaylistModal, ShowAddToNewPlaylistModal } = this.state;
        const { ContextType, Music, LikedMusics } = this.props;
        const isAvailable = Music.FilePath !== undefined;
        const LikeButtonAccessory = (
            <td className="align-middle">
                {Music ? (
                    <LikeButton
                        onLike={this.HandleLike}
                        defaultLikeState={LikedMusics ? LikedMusics.includes(Music._id) : false}
                    />
                ) : undefined}
            </td>
        );

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
                    ImageDz={Music.image_url}
                    Title={Music.title}
                    Artist={Music.artist_name}
                    onClick={this.onClick}
                    isAvailable={isAvailable}
                    AccessoryRight={LikeButtonAccessory}
                >
                    <Dropdown.Item onClick={this.onClick}>Play</Dropdown.Item>
                    <Dropdown.Item onClick={this.HandleAdd}>Add to current playlist</Dropdown.Item>
                    <Dropdown.Divider />
                    <Dropdown.Item onClick={this.ShowAddToPlaylistModal}>
                        Add to playlist
                    </Dropdown.Item>
                    <Dropdown.Item onClick={this.ShowAddToNewPlaylistModal}>
                        Add to a new playlist
                    </Dropdown.Item>
                    <Dropdown.Divider />
                    <Dropdown.Item onClick={this.HandleLike}>Like</Dropdown.Item>
                    {ContextType === OWN_PLAYLIST_CONTEXT && (
                        <>
                            <Dropdown.Divider />
                            <Dropdown.Item onClick={this.HandleDeletePlaylistMusic}>
                                Delete
                            </Dropdown.Item>
                        </>
                    )}
                </MusicItemRow>
            </>
        );
    }
}

const MusicElement = connect(mapStateToProps, mapDispatchToProps)(MusicElementConnected);

export default withRouter(MusicElement);
