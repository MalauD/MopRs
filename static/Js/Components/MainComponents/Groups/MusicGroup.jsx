import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { SortableElement } from 'react-sortable-hoc';
import {
    ClearPlaylist as ClearPlaylistRedux,
    AddMultipleMusics as AddMultipleMusicsRedux,
    ChangePlayingMusic as ChangePlayingMusicRedux,
} from '../../../Actions/Action';
import MusicElement from '../../Elements/MusicElement';
import ButtonIcon from '../../Helper/ButtonIcon';
import MediaLayout from '../../Layout/MediaLayout';
import { DefaultActions } from '../../Items/Actions';
import SortableMusicContainer from './SortableMusicContainer';

const mapDispatchToProps = (dispatch) => ({
    ClearPlaylist: () => {
        dispatch(ClearPlaylistRedux());
    },
    AddMusics: (Musics) => {
        dispatch(AddMultipleMusicsRedux(Musics));
    },
    ChangePlayingMusic: (Music) => {
        dispatch(ChangePlayingMusicRedux(Music));
    },
});

class MusicGroupConnected extends React.Component {
    static propTypes = {
        ClearPlaylist: PropTypes.func.isRequired,
        ChangePlayingMusic: PropTypes.func.isRequired,
        AddMusics: PropTypes.func.isRequired,
        Musics: PropTypes.arrayOf(PropTypes.shape({ _id: PropTypes.number.isRequired })).isRequired,
        isLoading: PropTypes.bool,
        title: PropTypes.string.isRequired,
        showMore: PropTypes.bool,
        onMoreClick: PropTypes.func,
        Actions: PropTypes.func,
        Accessories: PropTypes.arrayOf(PropTypes.element),
        AllowSort: PropTypes.bool,
        AlwaysSort: PropTypes.bool,
        DisplayActionsOnSort: PropTypes.bool,
        OnMusicElementClick: PropTypes.func,
        HighlightedMusics: PropTypes.arrayOf(PropTypes.number),
    };

    static defaultProps = {
        isLoading: false,
        showMore: false,
        onMoreClick: () => {},
        Actions: DefaultActions,
        Accessories: [],
        AllowSort: false,
        AlwaysSort: false,
        DisplayActionsOnSort: false,
        OnMusicElementClick: undefined,
        HighlightedMusics: [],
    };

    constructor(props) {
        super(props);
        this.state = {
            isInSorting: props.AlwaysSort,
        };
    }

    onPlayAll = () => {
        const { ClearPlaylist, AddMusics, Musics } = this.props;
        ClearPlaylist();
        AddMusics(Musics);
    };

    onAddAll = () => {
        const { AddMusics, Musics } = this.props;
        AddMusics(Musics);
    };

    onToggleSort = () => {
        this.setState((prevState) => ({
            isInSorting: !prevState.isInSorting,
        }));
    };

    render() {
        const {
            Musics,
            Actions,
            Accessories,
            AllowSort,
            DisplayActionsOnSort,
            OnMusicElementClick,
            ChangePlayingMusic,
            HighlightedMusics,
            ...props
        } = this.props;
        const { isInSorting } = this.state;

        const MusicElementSortable = SortableElement(({ value }) => (
            <MusicElement
                UseDragHandle
                ShowLikeButton={DisplayActionsOnSort}
                Music={value.Music}
                onClick={() =>
                    OnMusicElementClick
                        ? OnMusicElementClick(value.Music, value.index)
                        : ChangePlayingMusic(value.Music)
                }
                Actions={DisplayActionsOnSort ? Actions : undefined}
                Highlight={HighlightedMusics.includes(value.index)}
                Index={value.index}
                {...props}
            />
        ));

        const MusicItems = Musics.map((m, index) =>
            isInSorting ? (
                <MusicElementSortable key={m._id} index={index} value={{ Music: m, index }} />
            ) : (
                <MusicElement
                    key={m._id}
                    Music={m}
                    Actions={Actions}
                    onClick={() =>
                        OnMusicElementClick
                            ? OnMusicElementClick({ m, index })
                            : ChangePlayingMusic(m)
                    }
                    Highlight={HighlightedMusics.includes(index)}
                    Index={index}
                    {...props}
                />
            )
        );

        const accessories = [
            ...Accessories,
            <ButtonIcon
                dataEva="plus-circle-outline"
                evaOptions={{ fill: '#d6d6d6ff', width: '30px', height: '30px' }}
                onClick={this.onAddAll}
            />,
            <ButtonIcon
                dataEva="play-circle-outline"
                evaOptions={{ fill: '#d6d6d6ff', width: '30px', height: '30px' }}
                onClick={this.onPlayAll}
            />,
        ];

        if (AllowSort) {
            accessories.push(
                <ButtonIcon
                    dataEva={isInSorting ? 'edit' : 'edit-outline'}
                    evaOptions={{ fill: '#d6d6d6ff', width: '30px', height: '30px' }}
                    onClick={this.onToggleSort}
                />
            );
        }

        return isInSorting ? (
            <SortableMusicContainer accessories={accessories} {...props}>
                {MusicItems}
            </SortableMusicContainer>
        ) : (
            <MediaLayout accessories={accessories} {...props}>
                <table className="table table-hover table-borderless">
                    <tbody>{MusicItems}</tbody>
                </table>
            </MediaLayout>
        );
    }
}

const MusicGroup = connect(null, mapDispatchToProps)(MusicGroupConnected);

export default MusicGroup;
