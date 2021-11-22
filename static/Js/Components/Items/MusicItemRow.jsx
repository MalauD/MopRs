import React from 'react';
import PropTypes from 'prop-types';
import { Col, Image as ImgBootstrap, Row } from 'react-bootstrap';
import MoreButtonMusic from './Helper/MoreButtonMusic';
import { SortableHandle } from 'react-sortable-hoc';
import Icon from '../Helper/Icon';

const DragHandle = SortableHandle(() => (
	<Icon
		dataEva="menu-outline"
		evaOptions={{fill: "#d6d6d6ff", width: '30px', height: '30px'}}
	/>
));

const MusicItemRow = ({ onClick, Image, ImageDz, Title, Artist, children, isAvailable, AccessoryRight, UseDragHandle }) => (
	<tr className="w-100 m-0 p-0 PointerCursor MusicItemRow">
		<td className="p-0 py-3 pl-2 align-middle" style={{
			width: '10px'
		}}>
			{UseDragHandle && <DragHandle />}
		</td>

		<td className="p-0 py-3 pl-2 align-middle" onClick={onClick} style={{ width: '50px' }}>
			{ImageDz ? (
				<ImgBootstrap className="PlayerImage my-auto" rounded height="50em" src={ImageDz} />
			) : (
				<ImgBootstrap className="PlayerImage my-auto" rounded height="50em" src={Image ? `data:image/jpeg;base64,${Image.toString('base64')}` : '/Ressources/noMusic.jpg'} />
			)}
		</td>
		<td className="p-0 py-3 pl-3 align-top" onClick={onClick}>
			<Col className="pl-0">
				<Row className="p-0 m-0 pt-1">
					<h6 className={isAvailable ? 'p-0 m-0' : 'p-0 m-0 font-italic'}>{Title}</h6>
				</Row>
				<Row className="p-0 m-0">
					<p className="text-middle p-0 m-0">{Artist}</p>
				</Row>
			</Col>
		</td>

		{AccessoryRight}

		<td className="align-middle pr-4 Accessory">
			<MoreButtonMusic>{children}</MoreButtonMusic>
		</td>
	</tr>
);

MusicItemRow.propTypes = {
	onClick: PropTypes.func.isRequired,
	Image: PropTypes.string,
	ImageDz: PropTypes.string,
	Title: PropTypes.string.isRequired,
	Artist: PropTypes.string.isRequired,
	children: PropTypes.oneOfType([PropTypes.arrayOf(PropTypes.node), PropTypes.node]),
	AccessoryRight: PropTypes.node,
	isAvailable: PropTypes.bool.isRequired,
};

MusicItemRow.defaultProps = {
	Image: undefined,
	ImageDz: undefined,
	children: <></>,
	AccessoryRight: <></>,
};

export default MusicItemRow;
