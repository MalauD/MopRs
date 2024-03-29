import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import { PlayNext as PlayNextRedux } from '../../../Actions/Action';

const mapDispatchToProps = (dispatch) => ({
    PlayNext: (Music) => {
        dispatch(PlayNextRedux(Music));
    },
});

class PlayNextMusicAction extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({}).isRequired,
        PlayNext: PropTypes.func.isRequired,
        OnMusicPlayNext: PropTypes.func,
    };

    static defaultProps = {
        OnMusicPlayNext: () => {},
    };

    onClick = () => {
        const { PlayNext, Music, OnMusicPlayNext } = this.props;

        PlayNext(Music);
        OnMusicPlayNext(Music);
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Play next</Dropdown.Item>;
    }
}

export default connect(null, mapDispatchToProps)(PlayNextMusicAction);
