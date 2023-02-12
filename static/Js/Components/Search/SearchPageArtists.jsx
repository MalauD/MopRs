import QueryString from 'query-string';
import Axios from 'axios';
import React from 'react';
import PropTypes from 'prop-types';
import ArtistGroup from '../MainComponents/Groups/ArtistGroup';

class SearchPageArtists extends React.Component {
    static propTypes = {
        location: PropTypes.shape({
            search: PropTypes.string.isRequired,
        }).isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {
            Artists: [],
            PrevPageEmpty: false,
            CurrentPage: 0,
            IsFetchingArtists: false,
            PrevSearch: undefined,
        };
    }

    SearchArtists = () => {
        const { location } = this.props;

        const { IsFetchingArtists, PrevSearch } = this.state;

        const values = QueryString.parse(location.search);

        if (values.q !== PrevSearch && !IsFetchingArtists) {
            this.setState({ IsFetchingArtists: true });
            Axios.get(`/api/search/artist/${values.q}?maxResults=14&page=0`).then((res) => {
                this.setState({
                    Artists: res.data,
                    IsFetchingArtists: false,
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

        Axios.get(`/api/search/artist/${values.q}?maxResults=14&page=${CurrentPage + 1}`).then(
            (res) => {
                this.setState((prevState) => ({
                    Artists: [...prevState.Artists, ...res.data],
                    CurrentPage: prevState.CurrentPage + 1,
                    PrevPageEmpty: res.data.length === 0,
                }));
            }
        );
    };

    render() {
        const { Artists, IsFetchingArtists, PrevPageEmpty } = this.state;

        return (
            <ArtistGroup
                Artists={Artists}
                title="Artists"
                isLoading={IsFetchingArtists}
                showMore={!PrevPageEmpty}
                onMoreClick={this.OnMoreClick}
            />
        );
    }
}
export default SearchPageArtists;
