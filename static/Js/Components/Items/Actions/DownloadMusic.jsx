import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import fileDownload from 'js-file-download';

export default class DownloadMusic extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({
            _id: PropTypes.number.isRequired,
        }).isRequired,
    };

    onClick = () => {
        const { Music } = this.props;
        Axios.get(`/Music/Download/${Music._id}.mp3`, { responseType: 'blob' })
            .then((response) => {
                fileDownload(response.data, `${Music._id}.mp3`);
            });
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Download Music</Dropdown.Item>;
    }
}
