import React from 'react';
import PropTypes from 'prop-types';
import UserPlaylistElement from '../../Elements/UserPlaylistElement';
import MediaLayout from '../../Layout/MediaLayout';

class UserPlaylistGroup extends React.Component {
    static propTypes = {
        Playlists: PropTypes.arrayOf(
            PropTypes.shape({
                _id: PropTypes.string.isRequired,
            })
        ).isRequired,
        OnPlaylistDelete: PropTypes.func,
    };

    static defaultProps = {
        OnPlaylistDelete: () => {},
    };

    render() {
        const { Playlists, OnPlaylistDelete, ...props } = this.props;

        const PlaylistItems = Playlists.map((Playlist) => (
            <UserPlaylistElement
                key={Playlist._id}
                Playlist={Playlist}
                OnDelete={OnPlaylistDelete}
            />
        ));

        return (
            <MediaLayout {...props}>
                <div className="card-deck">{PlaylistItems}</div>
            </MediaLayout>
        );
    }
}

export default UserPlaylistGroup;
