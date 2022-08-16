import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import MusicGroup from './Groups/MusicGroup';

class Album extends React.Component {
    static propTypes = {
        match: PropTypes.shape({
            params: PropTypes.shape({
                id: PropTypes.string.isRequired,
            }).isRequired,
        }).isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {
            Musics: undefined,
            AlbumImageDz: undefined,
            AlbumName: '',
        };
    }

    componentDidMount = () => {
        const { match } = this.props;

        Axios.get(`/Music/Album/id/${match.params.id}`).then((res) => {
            this.setState({
                Musics: res.data.musics,
                AlbumName: res.data.name,
            });
        });
    };

    render() {
        const { Musics, AlbumName } = this.state;

        if (Musics) {
            return <MusicGroup Musics={Musics} title={AlbumName} />;
        }

        return <></>;
    }
}

export default Album;
