import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import MusicGroup from './Groups/MusicGroup';

class Trending extends React.Component {
    static propTypes = {
        Size: PropTypes.number,
        RemoveDups: PropTypes.bool,
    };

    static defaultProps = {
        Size: 14,
        RemoveDups: true,
    };

    constructor(props) {
        super(props);
        this.state = {
            Musics: undefined,
            PrevPageEmpty: false,
            CurrentPage: 0,
        };
    }

    componentDidMount() {
        const { Size } = this.props;

        Axios.get(`/api/trending/musics?page=0&maxResults=${Size}`).then((res) => {
            this.setState({
                Musics: res.data,
            });
        });
    }

    OnMoreClick = () => {
        const { Size, RemoveDups } = this.props;
        const { CurrentPage } = this.state;

        Axios.get(`/api/trending/musics?maxResults=${Size}&page=${CurrentPage + 1}`).then((res) => {
            this.setState((prevState) => ({
                Musics: [...prevState.Musics, ...(RemoveDups ? [...new Set(res.data)] : res.data)],
                CurrentPage: prevState.CurrentPage + 1,
                PrevPageEmpty: res.data.length === 0,
            }));
        });
    };

    render() {
        const { Musics, PrevPageEmpty } = this.state;

        if (Musics) {
            return (
                <MusicGroup
                    Musics={Musics}
                    title="Trending"
                    showMore={!PrevPageEmpty}
                    onMoreClick={this.OnMoreClick}
                />
            );
        }

        return null;
    }
}
export default Trending;
