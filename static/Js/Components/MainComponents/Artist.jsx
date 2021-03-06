import React from 'react';
import PropTypes from 'prop-types';
import Axios from 'axios';
import AlbumGroup from './Groups/AlbumGroup';
import ArtistGroup from './Groups/ArtistGroup';
import MusicGroup from './Groups/MusicGroup';
import { SEARCH_CONTEXT } from '../../Constants/MusicsConstants';

class Artist extends React.Component {
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
            ArtistId: undefined,
            ArtistName: '',
            AlbumsId: [],
            RelatedArtists: [],
            TopTracks: [],
            displayedTopTracks: 0,
            displayedAlbums: 0,
            displayedRelArtists: 0,
            isFetching: false,
        };
    }

    componentWillReceiveProps(nextProps) {
        const { ArtistId } = this.state;
        const NewArtistId = nextProps.match.params.id;

        if (ArtistId != NewArtistId && ArtistId !== undefined) {
            this.fetchArtist(NewArtistId);
        }
    }

    componentDidMount = () => {
        const ArtistId = this.props.match.params.id;
        this.fetchArtist(ArtistId);
    };

    fetchArtist = (artist_id) => {
        const { match } = this.props;
        this.setState({ isFetching: true });
        Axios.get(`/Music/Artist/id/${artist_id}`).then((res) => {
            this.setState({
                ArtistId: artist_id,
                ArtistName: res.data.name,
                AlbumsId: res.data.albums,
                RelatedArtists: res.data.related_artists,
                TopTracks: res.data.top_tracks,
                displayedTopTracks: Math.min(res.data.top_tracks.length, 8),
                displayedAlbums: Math.min(res.data.albums.length, 8),
                displayedRelArtists: Math.min(res.data.related_artists.length, 8),
                isFetching: false,
            });
        });
    };

    render() {
        const {
            ArtistName,
            RelatedArtists,
            TopTracks,
            AlbumsId,
            displayedTopTracks,
            displayedAlbums,
            displayedRelArtists,
            isFetching,
        } = this.state;

        return (
            <>
                <h2
                    style={{
                        fontFamily: 'Pacifico, cursive',
                        textShadow: '#cc506c 3px 3px 0px',
                    }}
                    className="align-self-center text-center"
                >
                    {ArtistName}
                </h2>
                <MusicGroup
                    Musics={TopTracks.slice(0, displayedTopTracks)}
                    DetailType="Top Musics"
                    ContextType={SEARCH_CONTEXT}
                    MoreButton={TopTracks.length > displayedTopTracks}
                    OnMoreClick={() => {
                        this.setState({ displayedTopTracks: displayedTopTracks + 8 });
                    }}
                    isFetching={isFetching}
                />
                <AlbumGroup
                    Albums={AlbumsId.slice(0, displayedAlbums)}
                    DetailType={'Albums'}
                    MoreButton={AlbumsId.length > displayedAlbums}
                    OnMoreClick={() => {
                        this.setState({ displayedAlbums: displayedAlbums + 8 });
                    }}
                    isFetching={isFetching}
                />
                <ArtistGroup
                    Artists={RelatedArtists.slice(0, displayedRelArtists)}
                    DetailType="Related Artists"
                    MoreButton={RelatedArtists.length > displayedRelArtists}
                    OnMoreClick={() => {
                        this.setState({ displayedRelArtists: displayedRelArtists + 8 });
                    }}
                    isFetching={isFetching}
                />
            </>
        );
    }
}

export default Artist;
