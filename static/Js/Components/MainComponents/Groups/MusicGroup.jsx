import React from 'react';
import {
	Button, Col, Row, Spinner,
} from 'react-bootstrap';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { ClearPlaylist as ClearPlaylistRedux, AddMultipleMusics as AddMultipleMusicsRedux } from '../../../Actions/Action';
import MusicElement from '../../Elements/MusicElement';
import ButtonIcon from '../../Helper/ButtonIcon';

const mapDispatchToProps = (dispatch) => ({
	ClearPlaylist: () => {
		dispatch(ClearPlaylistRedux());
	},
	AddMusics: (Musics) => {
		dispatch(AddMultipleMusicsRedux(Musics));
	},
});

class MusicGroupConnected extends React.Component {
	static propTypes = {
		ClearPlaylist: PropTypes.func.isRequired,
		AddMusics: PropTypes.func.isRequired,
		Musics: PropTypes.arrayOf(PropTypes.any).isRequired,
		DetailType: PropTypes.string.isRequired,
		IsFetching: PropTypes.bool,
		ContextType: PropTypes.string.isRequired,
		ContextPlaylistId: PropTypes.string,
		CommonImage: PropTypes.string,
		CommonImageDz: PropTypes.string,
		MoreButton: PropTypes.bool,
		OnMoreClick: PropTypes.func,
	}

	static defaultProps = {
		IsFetching: false,
		ContextPlaylistId: undefined,
		CommonImage: undefined,
		CommonImageDz: undefined,
		MoreButton: false,
		OnMoreClick: () => {},
	}

	onPlayAll = () => {
		const { ClearPlaylist, AddMusics, Musics } = this.props;
		ClearPlaylist();
		AddMusics(Musics);
	};

	render() {
		const {
			Musics,
			DetailType,
			IsFetching,
			ContextType,
			ContextPlaylistId,
			CommonImage,
			CommonImageDz,
			MoreButton,
			OnMoreClick,
		} = this.props;

		const MusicItems = Musics
			.map((m) => {
				const Music = m;
				if (CommonImage || CommonImageDz) {
					Music.AlbumId = {
						Image: CommonImage,
						ImagePathDeezer: CommonImageDz,
					};
				}

				return (
					<MusicElement
						key={Music._id}
						Music={Music}
						ContextType={ContextType}
						ContextPlaylistId={ContextPlaylistId}
					/>
				);
			});

		// TODO add empty graphic here

		if (IsFetching) {
			return (
				<div className="m-5">
					<small className="text-muted">
						<h5>Musics</h5>
					</small>
					<Spinner animation="border" role="status" size="lg">
						<span className="sr-only">Loading...</span>
					</Spinner>
				</div>
			);
		}

		return (
			<div className="m-4">
				<Row className="p-1">
					<Col className="my-auto">
						<small className="text-muted">
							<h5>{DetailType}</h5>
						</small>
					</Col>
					<Col className="">
						<ButtonIcon
						 dataEva={"play-circle-outline"}
						 evaOptions={{fill: "#d6d6d6ff", width: '30px', height: '30px'}} 
						 onClick={this.onPlayAll}
						 buttonClass="py-auto pr-4 float-right"
						 />
					</Col>
				</Row>
				<table className="table table-hover table-borderless">
					<tbody>{MusicItems}</tbody>
				</table>
				{MoreButton && (
					<div style={{ textAlign: 'center' }}>
						<Button onClick={OnMoreClick} variant="outline-dark">More</Button>
					</div>
				)}
			</div>
		);
	}
}

const MusicGroup = connect(null, mapDispatchToProps)(MusicGroupConnected);

export default MusicGroup;
