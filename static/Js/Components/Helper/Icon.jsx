import React, { useEffect, useState } from 'react';
import PropTypes from 'prop-types';
import * as eva from 'eva-icons';

/* eslint react/no-danger: 0 */

function Icon({ dataEva, evaOptions, ...props }) {
    const [Svg, SetSvg] = useState(eva.icons[dataEva].toSvg(evaOptions));
    useEffect(() => {
        if (dataEva) SetSvg(eva.icons[dataEva].toSvg(evaOptions));
    }, [dataEva]);
    return <div dangerouslySetInnerHTML={{ __html: Svg }} {...props} />;
}

Icon.propTypes = {
    dataEva: PropTypes.string.isRequired,
    evaOptions: PropTypes.shape({}),
};

Icon.defaultProps = {
    evaOptions: {},
};

export default Icon;
