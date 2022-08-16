import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { AddMusic as AddMusicRedux } from '../../../Actions/Action';
import { Dropdown } from 'react-bootstrap';

const mapDispatchToProps = (dispatch) => ({
    AddMusic: (Music) => {
        dispatch(AddMusicRedux(Music));
    },
});

class AddMusicAction extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({}).isRequired,
        AddMusic: PropTypes.func.isRequired,
    };

    onClick = () => {
        const { AddMusic, Music } = this.props;

        AddMusic(Music);
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Add to current playlist</Dropdown.Item>;
    }
}

export default connect(null, mapDispatchToProps)(AddMusicAction);
