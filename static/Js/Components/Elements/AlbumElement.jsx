import React from 'react';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import LazyLoad from 'react-lazyload';
import { withRouter } from 'react-router-dom';
import { Dropdown } from 'react-bootstrap';
import AlbumItemCard from '../Items/AlbumItemCard';
import { AddMultipleMusics, ClearPlaylist as ClearPlaylistRedux } from '../../Actions/Action';

const mapDispatchToProps = (dispatch) => ({
    ClearPlaylist: () => {
        dispatch(ClearPlaylistRedux());
    },
    AddMusics: (Musics) => {
        dispatch(AddMultipleMusics(Musics));
    },
});

class AlbumElementConnected extends React.Component {
    static propTypes = {
        history: PropTypes.shape({
            push: PropTypes.func.isRequired,
        }).isRequired,
        Album: PropTypes.shape({
            _id: PropTypes.number,
            name: PropTypes.string,
            cover: PropTypes.string,
            musics: PropTypes.arrayOf(PropTypes.shape({})),
            is_complete: PropTypes.bool,
        }).isRequired,
        ClearPlaylist: PropTypes.func.isRequired,
        AddMusics: PropTypes.func.isRequired,
    };

    onClick = () => {
        const { history, Album } = this.props;
        history.push(`/Album/${Album._id}`);
    };

    OnAdd = () => {
        const { AddMusics, Album } = this.props;
        AddMusics(Album.musics);
        // TODO Album should have musics instead of music id
    };

    OnPlay = () => {
        const { AddMusics, ClearPlaylist, Album } = this.props;
        ClearPlaylist();
        AddMusics(Album.musics);
    };

    render() {
        const { Album } = this.props;

        return (
            <LazyLoad>
                <AlbumItemCard
                    ImageDz={Album.cover}
                    Name={Album.name}
                    IsComplete={Album.is_complete}
                    onClick={this.onClick}
                    MoreOptions
                >
                    <Dropdown.Item onClick={this.OnPlay}>Play</Dropdown.Item>
                    <Dropdown.Item onClick={this.OnAdd}>Add to current playlist</Dropdown.Item>
                </AlbumItemCard>
            </LazyLoad>
        );
    }
}

const AlbumElement = connect(null, mapDispatchToProps)(AlbumElementConnected);

export default withRouter(AlbumElement);
