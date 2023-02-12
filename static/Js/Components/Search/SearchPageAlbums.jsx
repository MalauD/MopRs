import QueryString from 'query-string';
import Axios from 'axios';
import React from 'react';
import PropTypes from 'prop-types';
import AlbumGroup from '../MainComponents/Groups/AlbumGroup';

class SearchPageAlbums extends React.Component {
    static propTypes = {
        location: PropTypes.shape({
            search: PropTypes.string.isRequired,
        }).isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {
            Albums: [],
            PrevPageEmpty: false,
            CurrentPage: 0,
            IsFetchingAlbums: false,
            PrevSearch: undefined,
        };
    }

    SearchAlbums = () => {
        const { location } = this.props;

        const { IsFetchingAlbums, PrevSearch } = this.state;

        const values = QueryString.parse(location.search);

        if (values.q !== PrevSearch && !IsFetchingAlbums) {
            Axios.get(`/api/search/album/${values.q}?page=0&maxResults=14`).then((res) => {
                this.setState({
                    Albums: res.data,
                    IsFetchingAlbums: false,
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

        Axios.get(`/api/search/album/${values.q}?maxResults=14&page=${CurrentPage + 1}`).then(
            (res) => {
                this.setState((prevState) => ({
                    Albums: [...prevState.Albums, ...res.data],
                    CurrentPage: prevState.CurrentPage + 1,
                    PrevPageEmpty: res.data.length === 0,
                }));
            }
        );
    };

    render() {
        const { Albums, IsFetchingAlbums, PrevPageEmpty } = this.state;

        return (
            <AlbumGroup
                Albums={Albums}
                title="Albums"
                isLoading={IsFetchingAlbums}
                showMore={!PrevPageEmpty}
                onMoreClick={this.OnMoreClick}
            />
        );
    }
}
export default SearchPageAlbums;
