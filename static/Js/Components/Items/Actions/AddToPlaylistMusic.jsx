import React from 'react';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import AddToPlaylistModal from '../../Helper/AddToPlaylistModal';

export default class AddToPlaylistAction extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({}).isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {
            showModal: false,
        };
    }

    onClick = () => {
        this.setState({ showModal: true });
    };

    closeModal = () => {
        this.setState({ showModal: false });
    };

    render() {
        const { Music } = this.props;
        const { showModal } = this.state;

        return (
            <>
                {showModal && <AddToPlaylistModal Music={Music} OnClose={this.closeModal} />}
                <Dropdown.Item onClick={this.onClick}>Add to playlist</Dropdown.Item>
            </>
        );
    }
}
