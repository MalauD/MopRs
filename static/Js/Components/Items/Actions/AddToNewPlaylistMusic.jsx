import React from 'react';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import PlaylistCreateModal from '../../Helper/PlaylistCreateModal';

export default class AddToNewPlaylistAction extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({
            _id: PropTypes.number.isRequired,
        }).isRequired,
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
