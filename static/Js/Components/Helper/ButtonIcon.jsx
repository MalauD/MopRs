import React, { useEffect, useState } from 'react';
import PropTypes from 'prop-types';
import * as eva from 'eva-icons';

const ButtonIcon = ({
	onClick, dataEva, evaOptions,buttonClass, iconFontSize, ...props
}) => {
	const [Svg, SetSvg] = useState(eva.icons[dataEva].toSvg());
	useEffect(() => {
		SetSvg(eva.icons[dataEva].toSvg({
			...evaOptions,
			animation: {
				type: 'pulse',
				hover: true,
			}
		}))
	}, [dataEva])
	return (
	<div className={`ButtonIcon ${buttonClass}`} type="button" onClick={onClick} {...props}>
		<div dangerouslySetInnerHTML={{__html: Svg}} />
	</div>
)};

ButtonIcon.propTypes = {
	onClick: PropTypes.func.isRequired,
	dataEva: PropTypes.string.isRequired,
	buttonClass: PropTypes.string,
	iconFontSize: PropTypes.string,
};

ButtonIcon.defaultProps = {
	buttonClass: '',
	iconFontSize: '1.5rem',
};

export default ButtonIcon;
