import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import MusicGroup from './Groups/MusicGroup';
import { OWN_PLAYLIST_CONTEXT, PLAYLIST_CONTEXT } from '../../Constants/MusicsConstants';
import { connect } from 'react-redux';
import RelatedMusics from './RelatedMusics';

const mapStateToProps = (state) => ({
    Account: state.UserAccountReducer.Account,
});

class UserPlaylistConnected extends React.Component {
    static propTypes = {
        match: PropTypes.shape({
            params: PropTypes.shape({
                id: PropTypes.string.isRequired,
            }).isRequired,
        }).isRequired,
        Account: PropTypes.shape().isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {
            Musics: undefined,
            PlaylistName: '',
            PlaylistId: '',
            CreatorName: '',
            OwnPlaylist: false,
        };
    }

    componentDidMount = () => {
        const { match, Account } = this.props;

        Axios.get(`/Music/Playlist/id/${match.params.id}`).then((res) => {
            this.setState({
                Musics: res.data.musics,
                PlaylistName: res.data.name,
                PlaylistId: res.data._id,
                CreatorName: res.data.creator.username,
                OwnPlaylist: Account._id === res.data.creator._id,
            });
        });
    };

    onAdd = (Music) => {
        const { PlaylistId } = this.state;
        Axios.post(`/Music/Playlist/id/${PlaylistId}/Add`, {
            MusicsId: [Music._id],
        }).then(() => {
            this.setState({
                Musics: [...this.state.Musics, Music],
            });
        });
    };

    render() {
        const { Musics, PlaylistName, CreatorName, OwnPlaylist, PlaylistId } = this.state;

        if (Musics) {
            return (
                <>
                    <MusicGroup
                        Musics={Musics}
                        DetailType={`${PlaylistName} by ${CreatorName}`}
                        ContextType={OwnPlaylist ? OWN_PLAYLIST_CONTEXT : PLAYLIST_CONTEXT}
                        ContextPlaylistId={PlaylistId}
                    />
                    {OwnPlaylist && (
                        <RelatedMusics Musics={Musics} OnAdd={this.onAdd}></RelatedMusics>
                    )}
                </>
            );
        }

        return <></>;
    }
}

const UserPlaylist = connect(mapStateToProps)(UserPlaylistConnected);

export default UserPlaylist;
