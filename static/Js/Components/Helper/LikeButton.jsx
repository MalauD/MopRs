import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import ButtonIcon from './ButtonIcon';
import { LikeMusic as LikeMusicRedux } from '../../Actions/Action';
import axios from 'axios';

const mapDispatchToProps = (dispatch) => ({
    LikeMusic: (MusicId) => dispatch(LikeMusicRedux(MusicId)),
});

const mapStateToProps = (state) => {
    const { UserAccountReducer } = state;
    return { LikedMusics: UserAccountReducer.Account.liked_musics };
};

class LikeButtonConnected extends React.Component {
    static propTypes = {
        MusicId: PropTypes.number.isRequired,
        LikedMusics: PropTypes.arrayOf(PropTypes.number).isRequired,
        LikeMusic: PropTypes.func.isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {
            IsLiked: props.LikedMusics.indexOf(props.MusicId) !== -1,
        };
    }

    onButtonClick = () => {
        const { LikeMusic, MusicId } = this.props;
        axios.get(`/Music/Like/Music/${MusicId}`).then(() => {
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

        return (
            <ButtonIcon
                onClick={this.onButtonClick}
                dataEva={IsLiked ? 'heart' : 'heart-outline'}
                evaOptions={{ fill: '#CC506C', width: '30px', height: '30px' }}
                buttonClass="float-right d-none d-lg-block Accessory LikeButton"
            />
        );
    }
}

const LikeButton = connect(mapStateToProps, mapDispatchToProps)(LikeButtonConnected);

export default LikeButton;
