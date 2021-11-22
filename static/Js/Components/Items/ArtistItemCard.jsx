import React from 'react';
import PropTypes from 'prop-types';
import { Card } from 'react-bootstrap';

const ArtistItemCard = ({
	onClick, ImagePath, Name,
}) => (
	<Card style={{ width: '18rem', cursor: 'pointer' }} className="m-2" onClick={onClick}>
		<Card.Img variant="top" src={ImagePath || '/Ressources/noMusic.jpg'} />
		<Card.Body>
			<Card.Title>{Name}</Card.Title>
		</Card.Body>
	</Card>
);

ArtistItemCard.propTypes = {
	onClick: PropTypes.func.isRequired,
	ImagePath: PropTypes.string,
	Name: PropTypes.string,
};

ArtistItemCard.defaultProps = {
	ImagePath: undefined,
	Name: 'Loading...',
};

export default ArtistItemCard;
