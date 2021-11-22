import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import LazyLoad from 'react-lazyload';
import { withRouter } from 'react-router-dom';
import { Dropdown } from 'react-bootstrap';
import AlbumItemCard from '../Items/AlbumItemCard';
import { AddMultipleMusics, ClearPlaylist as ClearPlaylistRedux } from '../../Actions/Action';

const mapDispatchToProps = (dispatch) => ({
	ClearPlaylist: () => {
		dispatch(ClearPlaylistRedux());
	},
	AddMusics: (Musics) => {
		dispatch(AddMultipleMusics(Musics));
	},
});

class AlbumElementConnected extends React.Component {
	static propTypes = {
		history: PropTypes.shape({
			push: PropTypes.func.isRequired,
		}).isRequired,
		Album: PropTypes.shape({
			_id: PropTypes.string,
			Name: PropTypes.string,
			Image: PropTypes.string,
			ImageFormat: PropTypes.string,
			ImagePathDeezer: PropTypes.string,
			MusicsId: PropTypes.arrayOf(PropTypes.any),
			IsComplete: PropTypes.bool,
		}).isRequired,
		ClearPlaylist: PropTypes.func.isRequired,
		AddMusics: PropTypes.func.isRequired,
	}

	onClick = () => {
		const { history, Album } = this.props;
		history.push(`/Album/${Album._id}`);
	};


	componentWillUnmount = () => {
		this.setState = () => {

		};
	}

	GetAlbumMusics = () => {
		const { Album } = this.props;
		const { MusicsId } = Album;

		MusicsId.forEach((value, index, arr) => {
			/* eslint no-param-reassign: "off" */
			arr[index].AlbumId = Album;
		}, MusicsId);

		return MusicsId;
	}

	OnAdd = () => {
		const { AddMusics } = this.props;
		AddMusics(this.GetAlbumMusics());
	}

	OnPlay = () => {
		const { AddMusics, ClearPlaylist } = this.props;
		ClearPlaylist();
		AddMusics(this.GetAlbumMusics());
	}

	render() {
		const { Album } = this.props;

		return (
			<LazyLoad>
				<AlbumItemCard
					Image={Album.Image}
					ImageFormat={Album.ImageFormat}
					ImageDz={Album.ImagePathDeezer}
					Name={Album.Name}
					IsComplete={Album.IsComplete}
					onClick={this.onClick}
					MoreOptions
				>
					<Dropdown.Item onClick={this.OnPlay}>Play</Dropdown.Item>
					<Dropdown.Item onClick={this.OnAdd}>Add to current playlist</Dropdown.Item>
				</AlbumItemCard>
			</LazyLoad>

		);
	}
}

const AlbumElement = connect(null, mapDispatchToProps)(AlbumElementConnected);

export default withRouter(AlbumElement);
