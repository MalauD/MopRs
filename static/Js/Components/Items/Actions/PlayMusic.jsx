import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { ChangePlayingMusic as ChangePlayingMusicRedux } from '../../../Actions/Action';
import { Dropdown } from 'react-bootstrap';

const mapDispatchToProps = (dispatch) => ({
    ChangePlayingMusic: (Music) => {
        dispatch(ChangePlayingMusicRedux(Music));
    },
});

class PlayMusicAction extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({}).isRequired,
        ChangePlayingMusic: PropTypes.func.isRequired,
    };

    onClick = () => {
        const { ChangePlayingMusic, Music } = this.props;

        ChangePlayingMusic(Music);
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Play</Dropdown.Item>;
    }
}

export default connect(null, mapDispatchToProps)(PlayMusicAction);
