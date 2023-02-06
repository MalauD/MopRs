import React from 'react';
import { withRouter } from 'react-router-dom';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { arrayMoveImmutable } from 'array-move';
import {
    AddMusic as AddMusicRedux,
    ChangePlayingId as ChangePlayingIdRedux,
    UpdateCurrentPlaylist as UpdateCurrentPlaylistRedux,
} from '../../Actions/Action';
import PlaylistSaverButton from '../Helper/PlaylistSaverButton';
import ButtonIcon from '../Helper/ButtonIcon';
import RelatedMusics from './RelatedMusics';
import MusicGroup from './Groups/MusicGroup';
import { CurrentPlaylistActions } from '../Items/Actions';

const mapStateToProps = (state) => ({
    Musics: state.MusicPlayerReducer.Playlist.Musics,
    CurrentPlayingId: state.MusicPlayerReducer.Playlist.PlayingId,
});

const mapDispatchToProps = (dispatch) => ({
    ChangePlayingId: (id) => {
        dispatch(ChangePlayingIdRedux(id));
    },
    UpdateCurrentPlaylist: (UpdatedMusics, UpdatedPlayingId) => {
        dispatch(UpdateCurrentPlaylistRedux(UpdatedMusics, UpdatedPlayingId));
    },
    AddMusic: (Music) => {
        dispatch(AddMusicRedux(Music));
    },
});

function shuffle(a) {
    for (let i = a.length - 1; i > 0; i -= 1) {
        const j = Math.floor(Math.random() * (i + 1));
        /* eslint-disable no-param-reassign */
        [a[i], a[j]] = [a[j], a[i]];
    }
    return a;
}

class PlayerFull extends React.Component {
    static propTypes = {
        ChangePlayingId: PropTypes.func.isRequired,
        UpdateCurrentPlaylist: PropTypes.func.isRequired,
        Musics: PropTypes.arrayOf(PropTypes.shape({ _id: PropTypes.number })).isRequired,
        CurrentPlayingId: PropTypes.number,
    };

    static defaultProps = {
        CurrentPlayingId: 0,
    };

    constructor(props) {
        super(props);
        this.state = {};
    }

    onSortEnd = ({ oldIndex, newIndex }) => {
        const { CurrentPlayingId, UpdateCurrentPlaylist, Musics } = this.props;

        // Keep the current playing music at the same position
        function CalculateNewId(oldId) {
            if (oldIndex === oldId) {
                return newIndex;
            }
            if (oldIndex < oldId && newIndex >= oldId) {
                return oldId - 1;
            }
            if (oldIndex > oldId && newIndex <= oldId) {
                return oldId + 1;
            }
            return oldId;
        }

        const newMusicsPlaylist = arrayMoveImmutable(Musics, oldIndex, newIndex);
        UpdateCurrentPlaylist(newMusicsPlaylist, CalculateNewId(CurrentPlayingId));
    };

    onShuffle = () => {
        const { CurrentPlayingId, UpdateCurrentPlaylist, Musics } = this.props;
        const oldCurrentPlaying = Musics[CurrentPlayingId];
        const newMusicsPlaylist = arrayMoveImmutable(Musics, 0, 0);
        shuffle(newMusicsPlaylist);
        UpdateCurrentPlaylist(
            newMusicsPlaylist,
            newMusicsPlaylist.findIndex((m) => m._id === oldCurrentPlaying._id)
        );
    };

    render() {
        const { Musics, CurrentPlayingId, ChangePlayingId } = this.props;

        const accessories = [
            <ButtonIcon
                dataEva="shuffle-2-outline"
                onClick={this.onShuffle}
                evaOptions={{
                    fill: '#d6d6d6ff',
                    width: '30px',
                    height: '30px',
                }}
            />,
            <PlaylistSaverButton MusicsId={Musics.map((m) => m._id)} />,
        ];

        return (
            <>
                <MusicGroup
                    title="Current Playlist"
                    AlwaysSort
                    DisplayActionsOnSort
                    onSortEnd={this.onSortEnd}
                    Musics={Musics}
                    accessories={accessories}
                    Actions={CurrentPlaylistActions}
                    OnMusicElementClick={(_, i) => ChangePlayingId(i)}
                    HighlightedMusics={[CurrentPlayingId]}
                />
                <RelatedMusics
                    MusicIds={Musics.map((m) => m._id)}
                    CurrentPlaylistTitle="current playlist"
                />
            </>
        );
    }
}

const PlayerFullConnected = connect(mapStateToProps, mapDispatchToProps)(PlayerFull);

export default withRouter(PlayerFullConnected);
