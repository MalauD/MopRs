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
        OnMusicAdded: PropTypes.func,
    };

    static defaultProps = {
        OnMusicAdded: () => {},
    };

    onClick = () => {
        const { PlayNext, Music, OnMusicAdded } = this.props;

        PlayNext(Music);
        OnMusicAdded(Music);
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Play Next</Dropdown.Item>;
    }
}

export default connect(null, mapDispatchToProps)(PlayNextMusicAction);
