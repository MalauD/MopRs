import React from 'react';
import PropTypes from 'prop-types';
import { Card } from 'react-bootstrap';
import MoreButton from './Helper/MoreButton';

const AlbumItemCard = ({
	onClick, Image, Name, Artist, ImageDz, children, MoreOptions, IsComplete,
}) => (
	<Card style={{ width: '18rem', cursor: 'pointer' }} className="m-2 AlbumItemCard">
		{MoreOptions && (
			<div className="MoreIconButton">
				<MoreButton>{children}</MoreButton>
			</div>
		)}
		{ImageDz ? <Card.Img variant="top" src={ImageDz} onClick={onClick} />
			: <Card.Img variant="top" src={Image ? `data:image/jpeg;base64,${Image.toString('base64')}` : '/Ressources/noMusic.jpg'} />}

		<Card.Body onClick={onClick}>
			<Card.Title style={{ textAlign: 'center', fontStyle: IsComplete ? 'normal' : 'italic' }}>{Name}</Card.Title>
			<Card.Text>{Artist}</Card.Text>
		</Card.Body>

	</Card>
);

AlbumItemCard.propTypes = {
	onClick: PropTypes.func.isRequired,
	Image: PropTypes.string,
	ImageDz: PropTypes.string,
	Name: PropTypes.string.isRequired,
	Artist: PropTypes.string,
	children: PropTypes.oneOfType([
		PropTypes.arrayOf(PropTypes.node),
		PropTypes.node,
	]),
	MoreOptions: PropTypes.bool,
	IsComplete: PropTypes.bool.isRequired,
};

AlbumItemCard.defaultProps = {
	Image: undefined,
	ImageDz: '',
	Artist: '', // TODO Pass artist
	children: <></>,
	MoreOptions: false,
};

export default AlbumItemCard;
