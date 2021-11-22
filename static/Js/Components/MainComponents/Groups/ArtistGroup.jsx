import React from 'react';
import {
	Button, Col, Row, Spinner,
} from 'react-bootstrap';
import PropTypes from 'prop-types';
import ArtistElement from '../../Elements/ArtistElement';

class ArtistGroup extends React.Component {
	static propTypes = {
		Artists: PropTypes.arrayOf(PropTypes.any).isRequired,
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
			Artists,
			DetailType,
			IsFetching,
			MoreButton,
			OnMoreClick,
		} = this.props;

		const ArtistItems = Artists
			.map((Artist) => <ArtistElement key={Artist._id} Artist={Artist} />);

		// TODO add empty graphic here

		if (IsFetching) {
			return (
				<div className="m-5">
					<small className="text-muted">
						<h5>Artist</h5>
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
					{ArtistItems}
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

export default ArtistGroup;
