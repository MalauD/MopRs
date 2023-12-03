import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';
import fileDownload from 'js-file-download';

export default class DownloadMusic extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({
            _id: PropTypes.number.isRequired,
            title: PropTypes.string.isRequired,
            artist_name: PropTypes.string.isRequired,
        }).isRequired,
    };

    onClick = () => {
        const { Music } = this.props;
        Axios.get(`/api/music/${Music._id}/audio_tagged`, { responseType: 'blob' }).then(
            (response) => {
                let ext = 'mp3';
                if (response.data.type === 'audio/flac') {
                    ext = 'flac';
                }
                fileDownload(response.data, `${Music.artist_name} - ${Music.title}.${ext}`);
            }
        );
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Download music</Dropdown.Item>;
    }
}
