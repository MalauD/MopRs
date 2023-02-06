import React from 'react';
import PropTypes from 'prop-types';
import MusicItemRow from '../Items/MusicItemRow';
import LikeButton from '../Helper/LikeButton';

export default class MusicElement extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({
            _id: PropTypes.number.isRequired,
            title: PropTypes.string.isRequired,
            artist_name: PropTypes.string.isRequired,
            file_path: PropTypes.string,
            image_url: PropTypes.string,
        }).isRequired,
        Actions: PropTypes.func,
        UseDragHandle: PropTypes.bool,
        ShowLikeButton: PropTypes.bool,
    };

    static defaultProps = {
        UseDragHandle: false,
        ShowLikeButton: true,
        Actions: undefined,
    };

    render() {
        const { Music, Actions, UseDragHandle, ShowLikeButton, ...props } = this.props;
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
                UseDragHandle={UseDragHandle}
                AccessoryRight={ShowLikeButton ? LikeButtonAccessory : null}
                {...props}
            >
                {Actions ? <Actions Music={Music} {...props} /> : null}
            </MusicItemRow>
        );
    }
}
