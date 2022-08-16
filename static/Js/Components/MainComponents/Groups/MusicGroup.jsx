import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import {
    ClearPlaylist as ClearPlaylistRedux,
    AddMultipleMusics as AddMultipleMusicsRedux,
} from '../../../Actions/Action';
import MusicElement from '../../Elements/MusicElement';
import ButtonIcon from '../../Helper/ButtonIcon';
import MediaLayout from '../../Layout/MediaLayout';
import { DefaultActions } from '../../Items/Actions';

const mapDispatchToProps = (dispatch) => ({
    ClearPlaylist: () => {
        dispatch(ClearPlaylistRedux());
    },
    AddMusics: (Musics) => {
        dispatch(AddMultipleMusicsRedux(Musics));
    },
});

class MusicGroupConnected extends React.Component {
    static propTypes = {
        ClearPlaylist: PropTypes.func.isRequired,
        AddMusics: PropTypes.func.isRequired,
        Musics: PropTypes.arrayOf(PropTypes.any).isRequired,
        isLoading: PropTypes.bool,
        title: PropTypes.string.isRequired,
        showMore: PropTypes.bool,
        onMoreClick: PropTypes.func,
        Actions: PropTypes.func,
        Accessories: PropTypes.arrayOf(PropTypes.any),
    };

    static defaultProps = {
        isLoading: false,
        showMore: false,
        onMoreClick: () => {},
        Actions: DefaultActions,
        Accessories: [],
    };

    onPlayAll = () => {
        const { ClearPlaylist, AddMusics, Musics } = this.props;
        ClearPlaylist();
        AddMusics(Musics);
    };

    render() {
        const { Musics, Actions, Accessories, ...props } = this.props;

        const MusicItems = Musics.map((m) => {
            return <MusicElement key={m._id} Music={m} Actions={Actions} {...props} />;
        });

        let accessories = [
            ...Accessories,
            <ButtonIcon
                dataEva={'play-circle-outline'}
                evaOptions={{ fill: '#d6d6d6ff', width: '30px', height: '30px' }}
                onClick={this.onPlayAll}
            />,
        ];

        return (
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
