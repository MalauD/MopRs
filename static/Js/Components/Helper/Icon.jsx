import React, { useEffect, useState } from 'react';
import PropTypes from 'prop-types';
import * as eva from 'eva-icons';

const Icon = ({
	dataEva, evaOptions,...props
}) => {
	const [Svg, SetSvg] = useState(eva.icons[dataEva].toSvg(evaOptions));
	useEffect(() => {
		if(dataEva)
			SetSvg(eva.icons[dataEva].toSvg(evaOptions))
	}, [dataEva])
	return (
		<div dangerouslySetInnerHTML={{__html: Svg}} {...props} />
)};

Icon.propTypes = {
	dataEva: PropTypes.string.isRequired,
};

export default Icon;
