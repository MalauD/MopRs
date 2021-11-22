import React from 'react';
import { withRouter } from 'react-router-dom';
import LazyLoad from 'react-lazyload';
import PropTypes from 'prop-types';
import ArtistItemCard from '../Items/ArtistItemCard';

class ArtistElement extends React.Component {
	static propTypes = {
		Artist: PropTypes.shape({
			_id: PropTypes.string,
			Name: PropTypes.string,
			ImagePath: PropTypes.string,
		}).isRequired,
		history: PropTypes.shape({
			push: PropTypes.func.isRequired,
		}).isRequired,
	}

	onCardClick = () => {
		const { Artist } = this.props;
		const { history } = this.props;
		history.push(`/Artist/${Artist._id}`);
	}

	render() {
		const { Artist } = this.props;
		return (
			<LazyLoad>
				<ArtistItemCard
					Name={Artist.Name}
					ImagePath={Artist.ImagePath || '/Ressources/noMusic.jpg'}
					onClick={this.onCardClick}
				/>
			</LazyLoad>
		);
	}
}

export default withRouter(ArtistElement);
