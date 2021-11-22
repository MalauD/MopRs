import React from 'react';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import Icon from './../../Helper/Icon'

const MoreIconButton = React.forwardRef(({ onClick }, ref) => (
	<div
		href=""
		ref={ref}
		onClick={(e) => {
			e.preventDefault();
			onClick(e);
		}}
		className="float-right"
	>
		<Icon dataEva="plus-outline" evaOptions={{fill: "#d6d6d6ff", width: '34px', height: '34px'}} />
	</div>
));

MoreIconButton.propTypes = {
	onClick: PropTypes.func.isRequired,
};

const MoreButtonMusic = ({
	children,
}) => (
	<Dropdown>
		<Dropdown.Toggle variant="success" id="dropdown-basic" as={MoreIconButton} />

		<Dropdown.Menu>
			{children}
		</Dropdown.Menu>

	</Dropdown>
);

MoreButtonMusic.propTypes = {
	children: PropTypes.oneOfType([
		PropTypes.arrayOf(PropTypes.node),
		PropTypes.node,
	]),
};

MoreButtonMusic.defaultProps = {
	children: <></>,
};

export default MoreButtonMusic;
