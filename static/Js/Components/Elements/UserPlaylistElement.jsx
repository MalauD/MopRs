import React from 'react';
import Axios from 'axios';
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

const mapStateToProps = (state) => {
    const { UserAccountReducer } = state;
    return { AccountId: UserAccountReducer.Account._id };
};

class UserPlaylistElementConnected extends React.Component {
    static propTypes = {
        history: PropTypes.shape({
            push: PropTypes.func.isRequired,
        }).isRequired,
        Playlist: PropTypes.shape({
            _id: PropTypes.any,
            creator: PropTypes.shape({
                username: PropTypes.string,
            }),
            name: PropTypes.string,
            musics: PropTypes.arrayOf(PropTypes.any),
        }).isRequired,
        AccountId: PropTypes.string.isRequired,
        ClearPlaylist: PropTypes.func.isRequired,
        AddMusics: PropTypes.func.isRequired,
        OnDelete: PropTypes.func.isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {};
    }

    onClick = () => {
        const { history, Playlist } = this.props;
        history.push(`/Playlist/${Playlist._id}`);
    };

    OnDelete = () => {
        const { Playlist, OnDelete } = this.props;
        Axios.delete(`/Music/Playlist/id/${Playlist._id}`).then(() => {
            // TODO it resets the player
            OnDelete(Playlist);
        });
    };

    OnAdd = () => {
        const { AddMusics, Playlist } = this.props;
        AddMusics(Playlist.musics);
    };

    OnPlay = () => {
        const { AddMusics, ClearPlaylist, Playlist } = this.props;
        ClearPlaylist();
        AddMusics(Playlist.musics);
    };

    render() {
        const { Playlist, AccountId } = this.props;
        return (
            <LazyLoad>
                <AlbumItemCard
                    ImageDz={Playlist.musics[0].image_url}
                    Name={Playlist.name}
                    onClick={this.onClick}
                    MoreOptions
                >
                    <Dropdown.Item onClick={this.OnPlay}>Play</Dropdown.Item>
                    <Dropdown.Item onClick={this.OnAdd}>Add to current playlist</Dropdown.Item>
                    {AccountId === Playlist.creator._id && (
                        <>
                            <Dropdown.Divider />
                            <Dropdown.Item onClick={this.OnDelete}>Delete</Dropdown.Item>
                        </>
                    )}
                </AlbumItemCard>
            </LazyLoad>
        );
    }
}

const UserPlaylistElement = connect(
    mapStateToProps,
    mapDispatchToProps
)(UserPlaylistElementConnected);

export default withRouter(UserPlaylistElement);
