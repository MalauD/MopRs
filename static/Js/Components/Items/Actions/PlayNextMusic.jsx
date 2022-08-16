import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { PlayNext as PlayNextRedux } from '../../../Actions/Action';
import { Dropdown } from 'react-bootstrap';

const mapDispatchToProps = (dispatch) => ({
    PlayNext: (Music) => {
        dispatch(PlayNextRedux(Music));
    },
});

class PlayNextMusicAction extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({}).isRequired,
        PlayNext: PropTypes.func.isRequired,
    };

    onClick = () => {
        const { PlayNext, Music } = this.props;

        PlayNext(Music);
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Play Next</Dropdown.Item>;
    }
}

export default connect(null, mapDispatchToProps)(PlayNextMusicAction);
