import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { arrayMoveImmutable } from 'array-move';
import { SortableElement } from 'react-sortable-hoc';
import PlaylistElement from '../Elements/PlaylistElement';
import {
    AddMusic as AddMusicRedux,
    ChangePlayingId as ChangePlayingIdRedux,
    UpdateCurrentPlaylist as UpdateCurrentPlaylistRedux,
} from '../../Actions/Action';
import PlaylistSaverButton from '../Helper/PlaylistSaverButton';
import ButtonIcon from '../Helper/ButtonIcon';
import RelatedMusics from '../MainComponents/RelatedMusics';
import SortableMusicContainer from './SortableMusicContainer';

const mapStateToProps = (state) => ({
    Musics: state.MusicPlayerReducer.Playlist.Musics,
    CurrentPlaying:
        state.MusicPlayerReducer.Playlist.Musics[state.MusicPlayerReducer.Playlist.PlayingId],
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

class PlaylistContainerConnected extends React.Component {
    static propTypes = {
        ChangePlayingId: PropTypes.func.isRequired,
        UpdateCurrentPlaylist: PropTypes.func.isRequired,
        Musics: PropTypes.arrayOf(PropTypes.shape({})).isRequired,
        CurrentPlaying: PropTypes.shape({
            _id: PropTypes.number.isRequired,
        }),
    };

    static defaultProps = {
        CurrentPlaying: undefined,
    };

    onSortEnd = ({ oldIndex, newIndex }) => {
        const { CurrentPlaying, UpdateCurrentPlaylist, Musics } = this.props;
        const newMusicsPlaylist = arrayMoveImmutable(Musics, oldIndex, newIndex);
        UpdateCurrentPlaylist(newMusicsPlaylist, newMusicsPlaylist.indexOf(CurrentPlaying));
    };

    onShuffle = () => {
        const { CurrentPlaying, UpdateCurrentPlaylist, Musics } = this.props;
        const newMusicsPlaylist = arrayMoveImmutable(Musics, 0, 0);
        shuffle(newMusicsPlaylist);
        UpdateCurrentPlaylist(newMusicsPlaylist, newMusicsPlaylist.indexOf(CurrentPlaying));
    };

    render() {
        const { Musics, CurrentPlaying, ChangePlayingId } = this.props;

        const PlaylistSortableElement = SortableElement(({ value }) => (
            <PlaylistElement
                UseDragHandle
                key={value._id}
                ChangePlayingId={() => ChangePlayingId(value.index)}
                Music={value}
                IsThisPlaying={CurrentPlaying._id === value._id}
            />
        ));

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
                <SortableMusicContainer
                    title="Current playlist"
                    accessories={accessories}
                    onSortEnd={this.onSortEnd}
                >
                    {Musics.map((value, index) => (
                        <PlaylistSortableElement
                            key={`item-${value._id}`}
                            index={index}
                            value={{ ...value, index }}
                        />
                    ))}
                </SortableMusicContainer>
                <RelatedMusics MusicIds={Musics.map((m) => m._id)} />
            </>
        );
    }
}

const PlaylistContainer = connect(mapStateToProps, mapDispatchToProps)(PlaylistContainerConnected);

export default PlaylistContainer;
