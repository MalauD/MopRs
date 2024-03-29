import QueryString from 'query-string';
import Axios from 'axios';
import React from 'react';
import PropTypes from 'prop-types';
import UserPlaylistGroup from '../MainComponents/Groups/UserPlaylistGroup';

class SearchPagePlaylists extends React.Component {
    static propTypes = {
        location: PropTypes.shape({
            search: PropTypes.string.isRequired,
        }).isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {
            Playlists: [],
            PrevPageEmpty: false,
            CurrentPage: 0,
            IsFetchingPlaylists: false,
            PrevSearch: undefined,
        };
    }

    componentDidMount() {
        this.SearchPlaylists();
    }

    componentDidUpdate() {
        this.SearchPlaylists();
    }

    SearchPlaylists = () => {
        const { location } = this.props;

        const { IsFetchingPlaylists, PrevSearch } = this.state;

        const values = QueryString.parse(location.search);

        if (values.q !== PrevSearch && !IsFetchingPlaylists) {
            this.setState({ IsFetchingPlaylists: true });
            Axios.get(`/api/search/playlist/${values.q}?maxResults=8&page=0`).then((res) => {
                this.setState({
                    Playlists: res.data,
                    IsFetchingPlaylists: false,
                    PrevSearch: values.q,
                    CurrentPage: 0,
                    PrevPageEmpty: res.data.length === 0,
                });
            });
        }
    };

    OnMoreClick = () => {
        const { location } = this.props;
        const { CurrentPage } = this.state;

        const values = QueryString.parse(location.search);

        Axios.get(`/api/search/playlist/${values.q}?maxResults=8&page=${CurrentPage + 1}`).then(
            (res) => {
                this.setState((prevState) => ({
                    Playlists: [...prevState.Playlists, ...res.data],
                    CurrentPage: prevState.CurrentPage + 1,
                    PrevPageEmpty: res.data.length === 0,
                }));
            }
        );
    };

    render() {
        const { Playlists, IsFetchingPlaylists, PrevPageEmpty } = this.state;

        return (
            <UserPlaylistGroup
                Playlists={Playlists}
                title="Playlists"
                isLoading={IsFetchingPlaylists}
                showMore={!PrevPageEmpty}
                onMoreClick={this.OnMoreClick}
            />
        );
    }
}
export default SearchPagePlaylists;
