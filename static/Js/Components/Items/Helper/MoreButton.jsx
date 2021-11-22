import React from 'react';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import Icon from '../../Helper/Icon';

const MoreIconButton = React.forwardRef(({ onClick }, ref) => (
	<div

		href=""
		ref={ref}
		onClick={(e) => {
			e.preventDefault();
			onClick(e);
		}}
		style={{
			width: '40px',
			height: '40px',
			borderRadius: '50%',
			background: '#d6d6d640',
			position: 'relative',
		}}
	>
		<Icon
			dataEva="more-vertical-outline"
			evaOptions={{fill: "#d6d6d6ff", width: '30px', height: '30px'}} 
			// data-eva-fill="#d6d6d6ff"
			// className="my-auto"
			style={{
				fontSize: '20px',
				position: 'absolute',
				left: '5.25px',
				top: '4px',
				mixBlendMode: 'hard-light',
			}}
		/>
	</div>
));

MoreIconButton.propTypes = {
	onClick: PropTypes.func.isRequired,
};

const MoreButton = ({
	children,
}) => (
	<Dropdown drop="left">
		<Dropdown.Toggle variant="success" id="dropdown-basic" as={MoreIconButton} />

		<Dropdown.Menu>
			{children}
		</Dropdown.Menu>

	</Dropdown>
);

MoreButton.propTypes = {
	children: PropTypes.oneOfType([
		PropTypes.arrayOf(PropTypes.node),
		PropTypes.node,
	]),
};

MoreButton.defaultProps = {
	children: <></>,
};

export default MoreButton;
