import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import MusicItemRow from '../Items/MusicItemRow';
import LikeButton from '../Helper/LikeButton';
import { ChangePlayingMusic as ChangePlayingMusicRedux } from '../../Actions/Action';

const mapDispatchToProps = (dispatch) => ({
    ChangePlayingMusic: (Music) => {
        dispatch(ChangePlayingMusicRedux(Music));
    },
});

class MusicElementConnected extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({
            _id: PropTypes.number.isRequired,
            title: PropTypes.string.isRequired,
            artist_name: PropTypes.string.isRequired,
            file_path: PropTypes.string,
            image_url: PropTypes.string,
        }).isRequired,
        Actions: PropTypes.func,
        ChangePlayingMusic: PropTypes.func.isRequired,
        UseDragHandle: PropTypes.bool,
        ShowLikeButton: PropTypes.bool,
    };

    static defaultProps = {
        UseDragHandle: false,
        ShowLikeButton: true,
        Actions: undefined,
    };

    render() {
        const { Music, ChangePlayingMusic, Actions, UseDragHandle, ShowLikeButton, ...props } =
            this.props;
        const LikeButtonAccessory = (
            <td className="align-middle">
                {Music ? <LikeButton MusicId={Music._id} /> : undefined}
            </td>
        );
        return (
            <MusicItemRow
                ImageDz={Music.image_url}
                Title={Music.title}
                Artist={Music.artist_name}
                onClick={() => ChangePlayingMusic(Music)}
                UseDragHandle={UseDragHandle}
                AccessoryRight={ShowLikeButton ? LikeButtonAccessory : null}
            >
                {Actions ? <Actions Music={Music} {...props} /> : null}
            </MusicItemRow>
        );
    }
}

export default connect(null, mapDispatchToProps)(MusicElementConnected);
