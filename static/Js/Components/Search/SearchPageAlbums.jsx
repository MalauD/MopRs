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
			this.setState({ IsFetchingAlbums: true });
			Axios.get(`/Music/Search/Album/Name/${values.q}?PerPage=8`).then((res) => {
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

		Axios.get(`/Music/Search/Album/Name/${values.q}?PerPage=8&Page=${CurrentPage + 1}`).then((res) => {
			this.setState((prevState) => ({
				Albums: [...prevState.Albums, ...res.data],
				CurrentPage: prevState.CurrentPage + 1,
				PrevPageEmpty: res.data.length === 0,
			}));
		});
	};

	componentDidMount = () => {
		this.SearchAlbums();
	};

	componentDidUpdate = () => {
		this.SearchAlbums();
	};

	render() {
		const { Albums, IsFetchingAlbums, PrevPageEmpty } = this.state;

		return <AlbumGroup Albums={Albums} DetailType="Albums" IsFetching={IsFetchingAlbums} MoreButton={!PrevPageEmpty} OnMoreClick={this.OnMoreClick} />;
	}
}
export default SearchPageAlbums;
