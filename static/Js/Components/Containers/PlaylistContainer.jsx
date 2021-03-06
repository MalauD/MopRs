import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { Row, Col } from 'react-bootstrap';
import PlaylistElement from '../Elements/PlaylistElement';
import {
    ChangePlayingId as ChangePlayingIdRedux,
    UpdateCurrentPlaylist as UpdateCurrentPlaylistRedux,
} from '../../Actions/Action';
import PlaylistSaverButton from '../Helper/PlaylistSaverButton';
import { arrayMoveImmutable } from 'array-move';
import { SortableContainer, SortableElement } from 'react-sortable-hoc';
import ButtonIcon from '../Helper/ButtonIcon';

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
});

function shuffle(a) {
    for (let i = a.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [a[i], a[j]] = [a[j], a[i]];
    }
    return a;
}

class PlaylistContainerConnected extends React.Component {
    static propTypes = {
        ChangePlayingId: PropTypes.func.isRequired,
        UpdateCurrentPlaylist: PropTypes.func.isRequired,
        Musics: PropTypes.array.isRequired,
        CurrentPlaying: PropTypes.shape({
            id: PropTypes.string,
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

        const PlaylistSortableContainer = SortableContainer(({ children }) => {
            return (
                <div className="m-4">
                    <small className="text-muted">
                        <Row className="p-1">
                            <Col xs={9} className="mr-auto">
                                <small className="text-muted">
                                    <h5>Current Playlist</h5>
                                </small>
                            </Col>
                            <Col xs={3}>
                                <Row>
                                    <Col md="auto" className="mr-auto" />
                                    <Col xs={1} className="mx-1">
                                        <ButtonIcon
                                            dataEva={'shuffle-2-outline'}
                                            onClick={this.onShuffle}
                                            evaOptions={{
                                                fill: '#d6d6d6ff',
                                                width: '30px',
                                                height: '30px',
                                            }}
                                        />
                                    </Col>
                                    <Col xs={1} className="mx-1">
                                        <PlaylistSaverButton MusicsId={Musics.map((m) => m._id)} />
                                    </Col>
                                </Row>
                            </Col>
                        </Row>
                    </small>
                    <table className="table table-hover table-borderless">
                        <tbody>{children}</tbody>
                    </table>
                </div>
            );
        });

        return (
            <PlaylistSortableContainer onSortEnd={this.onSortEnd} useDragHandle>
                {Musics.map((value, index) => (
                    <PlaylistSortableElement
                        key={`item-${value._id}`}
                        index={index}
                        value={{ ...value, index }}
                    />
                ))}
            </PlaylistSortableContainer>
        );
    }
}

const PlaylistContainer = connect(mapStateToProps, mapDispatchToProps)(PlaylistContainerConnected);

export default PlaylistContainer;
