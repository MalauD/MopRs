import React from 'react';
import PropTypes from 'prop-types';
import Axios from 'axios';
import UserPlaylistGroup from './Groups/UserPlaylistGroup';

class UserPlaylists extends React.Component {
    static propTypes = {
        match: PropTypes.shape({
            params: PropTypes.shape({
                id: PropTypes.string.isRequired,
            }).isRequired,
        }).isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {
            IsFetching: false,
            Playlists: [],
        };
    }

    componentDidMount = () => {
        const { match } = this.props;
        this.setState({
            IsFetching: true,
        });
        Axios.get(`/User/${match.params.id}/Playlists?page=0&maxResults=100`).then((res) => {
            this.setState({
                Playlists: res.data,
                IsFetching: false,
            });
        });
    };

    render() {
        const { Playlists, IsFetching } = this.state;

        return (
            <UserPlaylistGroup
                Playlists={Playlists}
                DetailType={`Playlists of ${Playlists[0] ? Playlists[0].creator.username : 'loading..'}`}
                IsFetching={IsFetching}
            />
        );
    }
}

export default UserPlaylists;
