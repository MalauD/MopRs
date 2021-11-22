import React from 'react';
import PropTypes from 'prop-types';
import ButtonIcon from './ButtonIcon';

class LikeButton extends React.Component {
	static propTypes = {
		onLike: PropTypes.func.isRequired,
		defaultLikeState: PropTypes.bool,
	}

	static defaultProps = {
		defaultLikeState: false,
	}

	constructor(props) {
		super(props);
		this.state = {
			IsLiked: props.defaultLikeState,
		};
	}

	onButtonClick = () => {
		const { onLike } = this.props;
		this.setState((prevState) => ({
			IsLiked: !prevState.IsLiked,
		}), () => {
			const { IsLiked } = this.state;
			onLike(IsLiked);
		});
	}

	render() {
		const { IsLiked } = this.state;

		return (<ButtonIcon 
			onClick={this.onButtonClick}
			dataEva={IsLiked ? "heart" : "heart-outline"}
			evaOptions={{fill: "#CC506C", width: '30px', height: '30px'}} 
			buttonClass="float-right d-none d-lg-block Accessory LikeButton" 
		/>);
	}
}

export default LikeButton;
