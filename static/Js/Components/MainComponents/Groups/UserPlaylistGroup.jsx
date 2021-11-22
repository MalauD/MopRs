import React from 'react';
import {
	Button, Col, Row, Spinner,
} from 'react-bootstrap';
import PropTypes from 'prop-types';
import UserPlaylistElement from '../../Elements/UserPlaylistElement';

class UserPlaylistGroup extends React.Component {
	static propTypes = {
		Playlists: PropTypes.arrayOf(PropTypes.any).isRequired,
		DetailType: PropTypes.string.isRequired,
		IsFetching: PropTypes.bool,
		MoreButton: PropTypes.bool,
		OnMoreClick: PropTypes.func,
	}

	static defaultProps = {
		IsFetching: false,
		MoreButton: false,
		OnMoreClick: () => {},
	}

	constructor(props) {
		super(props);
		this.state = {

		};
	}

	render() {
		const {
			Playlists,
			DetailType,
			IsFetching,
			MoreButton,
			OnMoreClick,
		} = this.props;

		const PlaylistItems = Playlists
			.map((Playlist) => <UserPlaylistElement key={Playlist._id} Playlist={Playlist} />);

		// TODO add empty graphic here

		if (IsFetching) {
			return (
				<div className="m-5">
					<small className="text-muted">
						<h5>Playlists</h5>
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
					<Col>
						<small className="text-muted">
							<h5>{DetailType}</h5>
						</small>
					</Col>
				</Row>
				<div className="card-deck">
					{PlaylistItems}
				</div>
				{MoreButton && (
					<div style={{ textAlign: 'center' }}>
						<Button onClick={OnMoreClick} variant="outline-dark">More</Button>
					</div>
				)}
			</div>
		);
	}
}

export default UserPlaylistGroup;
