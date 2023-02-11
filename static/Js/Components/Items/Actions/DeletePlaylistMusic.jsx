import React from 'react';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';

export default class DeletePlaylistAction extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({}).isRequired,
        OnMusicPlaylistDelete: PropTypes.func.isRequired,
        Index: PropTypes.number.isRequired,
    };

    onClick = () => {
        const { Music, Index, OnMusicPlaylistDelete } = this.props;
        OnMusicPlaylistDelete({ Music, Index });
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Delete</Dropdown.Item>;
    }
}
