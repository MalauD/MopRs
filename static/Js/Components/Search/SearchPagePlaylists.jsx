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

	SearchPlaylists = () => {
		const { location } = this.props;

		const { IsFetchingPlaylists, PrevSearch } = this.state;

		const values = QueryString.parse(location.search);

		if (values.q !== PrevSearch && !IsFetchingPlaylists) {
			this.setState({ IsFetchingPlaylists: true });
			Axios.get(`/Music/Search/Playlist/Name/${values.q}?PerPage=8`).then((res) => {
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

		Axios.get(`/Music/Search/Playlist/Name/${values.q}?PerPage=8&Page=${CurrentPage + 1}`).then((res) => {
			this.setState((prevState) => ({
				Playlists: [...prevState.Playlists, ...res.data],
				CurrentPage: prevState.CurrentPage + 1,
				PrevPageEmpty: res.data.length === 0,
			}));
		});
	};

	componentDidMount = () => {
		this.SearchPlaylists();
	};

	componentDidUpdate = () => {
		this.SearchPlaylists();
	};

	render() {
		const { Playlists, IsFetchingPlaylists, PrevPageEmpty } = this.state;

		return <UserPlaylistGroup Playlists={Playlists} DetailType="Playlists" IsFetching={IsFetchingPlaylists} MoreButton={!PrevPageEmpty} OnMoreClick={this.OnMoreClick} />;
	}
}
export default SearchPagePlaylists;
