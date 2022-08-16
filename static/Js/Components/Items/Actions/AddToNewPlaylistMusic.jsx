import React from 'react';
import PropTypes from 'prop-types';
import PlaylistCreateModal from '../../Helper/PlaylistCreateModal';
import { Dropdown } from 'react-bootstrap';

export default class AddToNewPlaylistAction extends React.Component {
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
                {showModal && (
                    <PlaylistCreateModal MusicsId={[Music._id]} OnClose={this.closeModal} />
                )}
                <Dropdown.Item onClick={this.onClick}>Add to new playlist</Dropdown.Item>
            </>
        );
    }
}
