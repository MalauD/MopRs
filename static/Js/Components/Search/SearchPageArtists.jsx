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
			Axios.get(`/Music/Search/Artist/Name/${values.q}?PerPage=8`).then((res) => {
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

		Axios.get(`/Music/Search/Artist/Name/${values.q}?PerPage=8&Page=${CurrentPage + 1}`).then((res) => {
			this.setState((prevState) => ({
				Artists: [...prevState.Artists, ...res.data],
				CurrentPage: prevState.CurrentPage + 1,
				PrevPageEmpty: res.data.length === 0,
			}));
		});
	};

	componentDidMount = () => {
		this.SearchArtists();
	};

	componentDidUpdate = () => {
		this.SearchArtists();
	};

	render() {
		const { Artists, IsFetchingArtists, PrevPageEmpty } = this.state;

		return <ArtistGroup Artists={Artists} DetailType="Artists" IsFetching={IsFetchingArtists} MoreButton={!PrevPageEmpty} OnMoreClick={this.OnMoreClick} />;
	}
}
export default SearchPageArtists;
