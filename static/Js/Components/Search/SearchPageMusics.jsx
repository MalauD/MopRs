import QueryString from 'query-string';
import Axios from 'axios';
import React from 'react';
import PropTypes from 'prop-types';
import MusicGroup from '../MainComponents/Groups/MusicGroup';
import { SEARCH_CONTEXT } from '../../Constants/MusicsConstants';

class SearchPageMusics extends React.Component {
	static propTypes = {
		location: PropTypes.shape({
			search: PropTypes.string.isRequired,
		}).isRequired,
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

	SearchMusics = () => {
		const { location } = this.props;

		const { IsFetchingMusics, PrevSearch } = this.state;

		const values = QueryString.parse(location.search);

		if (values.q !== PrevSearch && !IsFetchingMusics) {
			this.setState({ IsFetchingMusics: true });
			Axios.get(`/Music/Search/Music/Name/${values.q}?PerPage=8`).then((res) => {
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

		Axios.get(`/Music/Search/Music/Name/${values.q}?PerPage=8&Page=${CurrentPage + 1}`).then((res) => {
			this.setState((prevState) => ({
				Musics: [...prevState.Musics, ...res.data],
				CurrentPage: prevState.CurrentPage + 1,
				PrevPageEmpty: res.data.length === 0,
			}));
		});
	};

	componentDidMount = () => {
		this.SearchMusics();
	};

	componentDidUpdate = () => {
		this.SearchMusics();
	};

	render() {
		const { Musics, IsFetchingMusics, PrevPageEmpty } = this.state;

		return <MusicGroup Musics={Musics} DetailType="Musics" IsFetching={IsFetchingMusics} ContextType={SEARCH_CONTEXT} MoreButton={!PrevPageEmpty} OnMoreClick={this.OnMoreClick} />;
	}
}
export default SearchPageMusics;
