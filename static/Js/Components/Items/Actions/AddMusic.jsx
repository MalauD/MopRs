import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import { AddMusic as AddMusicRedux } from '../../../Actions/Action';

const mapDispatchToProps = (dispatch) => ({
    AddMusic: (Music) => {
        dispatch(AddMusicRedux(Music));
    },
});

class AddMusicAction extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({}).isRequired,
        AddMusic: PropTypes.func.isRequired,
        OnMusicAdded: PropTypes.func,
    };

    static defaultProps = {
        OnMusicAdded: () => {},
    };

    onClick = () => {
        const { AddMusic, Music, OnMusicAdded } = this.props;

        AddMusic(Music);
        OnMusicAdded(Music);
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Add to current playlist</Dropdown.Item>;
    }
}

export default connect(null, mapDispatchToProps)(AddMusicAction);
