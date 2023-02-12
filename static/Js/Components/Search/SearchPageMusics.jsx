import QueryString from 'query-string';
import Axios from 'axios';
import React from 'react';
import PropTypes from 'prop-types';
import MusicGroup from '../MainComponents/Groups/MusicGroup';

class SearchPageMusics extends React.Component {
    static propTypes = {
        location: PropTypes.shape({
            search: PropTypes.string.isRequired,
        }).isRequired,
        onSearchEnd: PropTypes.func,
    };

    static defaultProps = {
        onSearchEnd: () => {},
    };

    constructor(props) {
        super(props);
        this.state = {
            Musics: [],
            PrevPageEmpty: false,
            CurrentPage: 0,
            IsFetchingMusics: false,
            PrevSearch: undefined,
        };
    }

    componentDidMount() {
        this.SearchMusics();
    }

    componentDidUpdate() {
        this.SearchMusics();
    }

    SearchMusics = () => {
        const { location, onSearchEnd } = this.props;

        const { IsFetchingMusics, PrevSearch } = this.state;

        const values = QueryString.parse(location.search);

        if (values.q !== PrevSearch && !IsFetchingMusics) {
            this.setState({ IsFetchingMusics: true });
            Axios.get(`/api/search/music/${values.q}?maxResults=14&page=0`).then((res) => {
                onSearchEnd();
                this.setState({
                    Musics: res.data,
                    IsFetchingMusics: false,
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

        Axios.get(`/api/search/music/${values.q}?maxResults=14&page=${CurrentPage + 1}`).then(
            (res) => {
                this.setState((prevState) => ({
                    Musics: [...prevState.Musics, ...res.data],
                    CurrentPage: prevState.CurrentPage + 1,
                    PrevPageEmpty: res.data.length === 0,
                }));
            }
        );
    };

    render() {
        const { Musics, IsFetchingMusics, PrevPageEmpty } = this.state;

        return (
            <MusicGroup
                Musics={Musics}
                title="Musics"
                isLoading={IsFetchingMusics}
                showMore={!PrevPageEmpty}
                onMoreClick={this.OnMoreClick}
            />
        );
    }
}
export default SearchPageMusics;
