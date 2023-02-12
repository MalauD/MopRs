import React from 'react';
import Axios from 'axios';
import PropTypes from 'prop-types';
import MusicGroup from './Groups/MusicGroup';

class Favorites extends React.Component {
    static propTypes = {
        Size: PropTypes.number,
    };

    static defaultProps = {
        Size: 14,
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

        Axios.get(`/api/me/likes/musics?maxResults=${Size}&page=0`).then((res) => {
            this.setState({
                Musics: res.data,
            });
        });
    }

    OnMoreClick = () => {
        const { Size } = this.props;
        const { CurrentPage } = this.state;

        Axios.get(`/api/me/likes/musics?maxResults=${Size}&page=${CurrentPage + 1}`).then((res) => {
            this.setState((prevState) => ({
                Musics: [...prevState.Musics, ...res.data],
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
                    title="Favorites"
                    showMore={!PrevPageEmpty}
                    onMoreClick={this.OnMoreClick}
                />
            );
        }

        return null;
    }
}
export default Favorites;
