import React from 'react';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';

export default class AddPlaylistMusicAction extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({}).isRequired,
        OnPlaylistMusicAdded: PropTypes.func,
        CurrentPlaylistTitle: PropTypes.string.isRequired,
    };

    static defaultProps = {
        OnPlaylistMusicAdded: () => {},
    };

    onClick = () => {
        const { Music, OnPlaylistMusicAdded } = this.props;

        OnPlaylistMusicAdded(Music);
    };

    render() {
        const { CurrentPlaylistTitle } = this.props;
        return <Dropdown.Item onClick={this.onClick}>Add to {CurrentPlaylistTitle}</Dropdown.Item>;
    }
}
