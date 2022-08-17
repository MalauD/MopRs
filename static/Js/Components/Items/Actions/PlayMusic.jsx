import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import { ChangePlayingMusic as ChangePlayingMusicRedux } from '../../../Actions/Action';

const mapDispatchToProps = (dispatch) => ({
    ChangePlayingMusic: (Music) => {
        dispatch(ChangePlayingMusicRedux(Music));
    },
});

class PlayMusicAction extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({}).isRequired,
        ChangePlayingMusic: PropTypes.func.isRequired,
        OnMusicAdded: PropTypes.func,
    };

    static defaultProps = {
        OnMusicAdded: () => {},
    };

    onClick = () => {
        const { ChangePlayingMusic, Music, OnMusicAdded } = this.props;

        ChangePlayingMusic(Music);
        OnMusicAdded(Music);
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Play</Dropdown.Item>;
    }
}

export default connect(null, mapDispatchToProps)(PlayMusicAction);
