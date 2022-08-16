import React from 'react';
import PropTypes from 'prop-types';
import AlbumElement from '../../Elements/AlbumElement';
import MediaLayout from '../../Layout/MediaLayout';

class AlbumGroup extends React.Component {
    static propTypes = {
        Albums: PropTypes.arrayOf(PropTypes.any).isRequired,
    };

    render() {
        const { Albums, ...props } = this.props;

        const AlbumItems = Albums.map((Album) => <AlbumElement key={Album._id} Album={Album} />);

        return (
            <MediaLayout {...props}>
                <div className="card-deck">{AlbumItems}</div>
            </MediaLayout>
        );
    }
}

export default AlbumGroup;
