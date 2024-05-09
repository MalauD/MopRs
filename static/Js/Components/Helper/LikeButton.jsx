import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import axios from 'axios';
import ButtonIcon from './ButtonIcon';
import { LikeMusic as LikeMusicRedux } from '../../Actions/Action';

const mapDispatchToProps = (dispatch) => ({
    LikeMusic: (MusicId) => dispatch(LikeMusicRedux(MusicId)),
});

const mapStateToProps = (state) => {
    const { UserAccountReducer } = state;
    if (!UserAccountReducer.Account) {
        return { LikedMusics: [] };
    }
    return { LikedMusics: UserAccountReducer.Account.liked_musics };
};

class LikeButtonConnected extends React.Component {
    static propTypes = {
        MusicId: PropTypes.number.isRequired,
        LikedMusics: PropTypes.arrayOf(PropTypes.number).isRequired,
        LikeMusic: PropTypes.func.isRequired,
        buttonClass: PropTypes.string,
    };

    static defaultProps = {
        buttonClass: 'float-right d-none d-lg-block Accessory LikeButton',
    };

    constructor(props) {
        super(props);
        this.state = {
            IsLiked: props.LikedMusics.indexOf(props.MusicId) !== -1,
        };
    }

    UNSAFE_componentWillReceiveProps(nextProps) {
        // Update IsLiked state if LikedMusics array changes or MusicId changes
        const { MusicId, LikedMusics } = nextProps;
        this.setState({
            IsLiked: LikedMusics.indexOf(MusicId) !== -1,
        });
    }

    onButtonClick = () => {
        const { LikeMusic, MusicId } = this.props;
        axios.get(`/api/music/${MusicId}/like`).then(() => {
            this.setState(
                (prevState) => ({
                    IsLiked: !prevState.IsLiked,
                }),
                () => {
                    LikeMusic(MusicId);
                }
            );
        });
    };

    render() {
        const { IsLiked } = this.state;
        const { buttonClass } = this.props;

        return (
            <ButtonIcon
                onClick={this.onButtonClick}
                dataEva={IsLiked ? 'heart' : 'heart-outline'}
                evaOptions={{ fill: '#CC506C', width: '30px', height: '30px' }}
                buttonClass={buttonClass}
                {...this.props}
            />
        );
    }
}

const LikeButton = connect(mapStateToProps, mapDispatchToProps)(LikeButtonConnected);

export default LikeButton;
