import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import MusicGroup from './Groups/MusicGroup';
import { FAV_CONTEXT } from '../../Constants/MusicsConstants';

class Favorites extends React.Component {
	static propTypes = {
		Size: PropTypes.number,
		Reverse: PropTypes.bool,
	}

	static defaultProps = {
		Size: undefined,
		Reverse: true,
	}

	constructor(props) {
		super(props);
		this.state = {
			Musics: undefined,
			PrevPageEmpty: false,
			CurrentPage: 0,
		};
	}

	componentDidMount() {
		const { Size, Reverse } = this.props;

		Axios.get('/User/LikedMusics?PerPage=8&Page=0').then((res) => {
			this.setState({
				Musics: res.data,
			});
		});
	}

	OnMoreClick = () => {
		const { CurrentPage } = this.state;

		Axios.get(`/User/LikedMusics?PerPage=8&Page=${CurrentPage + 1}`).then((res) => {
			this.setState((prevState) => ({
				Musics: [...prevState.Musics, ...res.data],
				CurrentPage: prevState.CurrentPage + 1,
				PrevPageEmpty: res.data.length === 0,
			}));
		});
	};

	render() {
		const { Musics, PrevPageEmpty } = this.state;

		if (Musics) {
			return (
				<MusicGroup
					Musics={Musics}
					DetailType="Favorites"
					ContextType={FAV_CONTEXT}
					MoreButton={!PrevPageEmpty}
					OnMoreClick={this.OnMoreClick}
				/>
			);
		}

		return <></>;
	}
}
export default Favorites;
