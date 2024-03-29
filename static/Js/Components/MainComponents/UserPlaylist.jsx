import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import { arrayMoveImmutable } from 'array-move';
import MusicGroup from './Groups/MusicGroup';
import RelatedMusics from './RelatedMusics';
import { DefaultActions, OwnPlaylistActions, OwnPlaylistRelatedActions } from '../Items/Actions';
import PlaylistModifyModal from '../Helper/PlaylistModifyModal';

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
            PlaylistPublic: false,
        };
    }

    componentDidMount() {
        const { match, Account } = this.props;

        Axios.get(`/api/playlist/${match.params.id}`).then((res) => {
            this.setState({
                Musics: res.data.musics,
                PlaylistName: res.data.name,
                PlaylistId: res.data._id,
                CreatorName: res.data.creator.username,
                OwnPlaylist: Account._id === res.data.creator._id,
                PlaylistPublic: res.data.public,
            });
        });
    }

    onAdd = (Music) => {
        const { PlaylistId } = this.state;
        Axios.post(`/api/playlist/${PlaylistId}/musics`, {
            MusicsId: [Music._id],
        }).then(() => {
            this.setState((prevState) => ({
                Musics: [...prevState.Musics, Music],
            }));
        });
    };

    onDelete = ({ Index }) => {
        const { PlaylistId, Musics } = this.state;
        Axios.delete(`/api/playlist/${PlaylistId}/musics`, {
            data: { AtIndex: Index },
        }).then(() => {
            this.setState({
                Musics: Musics.filter((m, i) => i !== Index),
            });
        });
    };

    onSortEnd = ({ oldIndex, newIndex }) => {
        const { Musics, PlaylistId } = this.state;
        const newMusicsPlaylist = arrayMoveImmutable(Musics, oldIndex, newIndex);
        this.setState({
            Musics: newMusicsPlaylist,
        });
        Axios.post(`/api/playlist/${PlaylistId}/musics/edit`, {
            MusicsId: newMusicsPlaylist.map((m) => m._id),
        })
            .then(() => {})
            .catch(() => {
                this.setState({
                    Musics,
                });
            });
    };

    OnPlaylistModify = ({ Name, IsPublic }) => {
        this.setState({ PlaylistName: Name, PlaylistPublic: IsPublic });
    };

    render() {
        const { Musics, PlaylistName, CreatorName, OwnPlaylist, PlaylistId, PlaylistPublic } =
            this.state;

        const Accessories = OwnPlaylist
            ? [
                  <PlaylistModifyModal
                      OldName={PlaylistName}
                      OldIsPublic={PlaylistPublic}
                      OnPlaylistModify={this.OnPlaylistModify}
                      PlaylistId={PlaylistId}
                  />,
              ]
            : undefined;

        if (Musics) {
            return (
                <>
                    <MusicGroup
                        Musics={Musics}
                        title={`${PlaylistName} by ${CreatorName}`}
                        Actions={OwnPlaylist ? OwnPlaylistActions : DefaultActions}
                        AllowSort={OwnPlaylist}
                        onSortEnd={this.onSortEnd}
                        OnMusicPlaylistDelete={this.onDelete}
                        Accessories={Accessories}
                    />
                    {OwnPlaylist && (
                        <RelatedMusics
                            MusicIds={Musics.map((m) => m._id)}
                            Actions={OwnPlaylistRelatedActions}
                            CurrentPlaylistTitle={PlaylistName}
                            OnPlaylistMusicAdded={this.onAdd}
                        />
                    )}
                </>
            );
        }

        return null;
    }
}

const UserPlaylist = connect(mapStateToProps)(UserPlaylistConnected);

export default UserPlaylist;
