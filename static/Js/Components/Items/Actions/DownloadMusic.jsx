import React from 'react';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';

export default class DownloadMusic extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({
            _id: PropTypes.number.isRequired,
        }).isRequired,
    };

    onClick = () => {
        const { Music } = this.props;
        window.open(`/Music/Download/${Music._id}.mp3`, '_blank');
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Download Music</Dropdown.Item>;
    }
}
