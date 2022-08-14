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
            Creator: {},
        };
    }

    componentDidMount = () => {
        const { match } = this.props;
        this.setState({
            IsFetching: true,
        });
        Axios.get(`/User/${match.params.id}/Playlists?page=0&maxResults=100`).then((res) => {
            this.setState({
                Playlists: res.data.Playlists,
                Creator: res.data.Creator,
                IsFetching: false,
            });
        });
    };

    render() {
        const { Playlists, IsFetching, Creator } = this.state;

        return (
            <UserPlaylistGroup
                Playlists={Playlists}
                DetailType={`Playlists of ${Creator.username}`}
                IsFetching={IsFetching}
            />
        );
    }
}

export default UserPlaylists;
