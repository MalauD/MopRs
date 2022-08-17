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
        Actions: PropTypes.func.isRequired,
        ChangePlayingMusic: PropTypes.func.isRequired,
    };

    render() {
        const { Music, ChangePlayingMusic, Actions, ...props } = this.props;
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
                AccessoryRight={LikeButtonAccessory}
            >
                <Actions Music={Music} {...props} />
            </MusicItemRow>
        );
    }
}

export default connect(null, mapDispatchToProps)(MusicElementConnected);
