import React from 'react';
import PropTypes from 'prop-types';
import ArtistElement from '../../Elements/ArtistElement';
import MediaLayout from '../../Layout/MediaLayout';

class ArtistGroup extends React.Component {
    static propTypes = {
        Artists: PropTypes.arrayOf(
            PropTypes.shape({
                _id: PropTypes.number.isRequired,
            })
        ).isRequired,
    };

    render() {
        const { Artists, ...props } = this.props;

        const ArtistItems = Artists.map((Artist) => (
            <ArtistElement key={Artist._id} Artist={Artist} />
        ));

        return (
            <MediaLayout {...props}>
                <div className="card-deck">{ArtistItems}</div>
            </MediaLayout>
        );
    }
}

export default ArtistGroup;
