import React, { useRef, useState } from 'react';
import Draggable from 'react-draggable';
import PropTypes from 'prop-types';
import useResize from './Helper/useResize';


const PlayerSlider = ({ Time, EndTime, OnSliderChange }) => {
	const ContainerRef = useRef(null);
	const { width } = useResize(ContainerRef);
	const [IsDragging, SetIsDragging] = useState(false);

	return (
		<div ref={ContainerRef} style={{ width: '100%', height: '2px' }} id="Progress-container" className="mb-2">
			<div
				className={`Progress-bar ${IsDragging ? 'noTransition' : ''}`}
				style={{ width: `${(Time * width) / EndTime}px`, height: '100%' }}
			/>

			<Draggable
				axis="x"
				bounds="parent"
				position={{
					x: (Time * width) / EndTime,
					y: 0,
				}}
				positionOffset={{
					x: -6,
					y: 0,
				}}
				onStart={() => SetIsDragging(true)}
				onDrag={(e, data) => {
					OnSliderChange((data.x * EndTime) / width);
				}}
				onStop={() => SetIsDragging(false)}
			>
				<div className={`Progress-drag  ${IsDragging ? 'noTransition' : ''}`}>
					<div className="Progress-ball" />
				</div>
			</Draggable>

		</div>
	);
};

PlayerSlider.propTypes = {
	Time: PropTypes.number.isRequired,
	EndTime: PropTypes.number.isRequired,
	OnSliderChange: PropTypes.func.isRequired,
};

export default PlayerSlider;
