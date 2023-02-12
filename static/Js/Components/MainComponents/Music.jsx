import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import MusicGroup from './Groups/MusicGroup';

class Music extends React.Component {
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
            MusicData: undefined,
        };
    }

    componentDidMount() {
        const { match } = this.props;

        Axios.get(`/api/music/${match.params.id}`).then((res) => {
            this.setState({
                MusicData: res.data,
            });
        });
    }

    componentDidUpdate(prevProps) {
        // Check if music id changed in url
        const { match } = this.props;
        if (match.params.id !== prevProps.match.params.id) {
            Axios.get(`/api/music/${match.params.id}`).then((res) => {
                this.setState({
                    MusicData: res.data,
                });
            });
        }
    }

    render() {
        const { MusicData } = this.state;

        if (MusicData) {
            return <MusicGroup Musics={[MusicData]} title={MusicData.title} />;
        }

        return null;
    }
}

export default Music;
