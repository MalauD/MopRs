import React from 'react';
import PropTypes from 'prop-types';
import Axios from 'axios';
import AlbumGroup from './Groups/AlbumGroup';
import ArtistGroup from './Groups/ArtistGroup';
import MusicGroup from './Groups/MusicGroup';

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

    componentDidMount() {
        const { match } = this.props;
        const ArtistId = match.params.id;
        this.fetchArtist(ArtistId);
    }

    UNSAFE_componentWillReceiveProps(nextProps) {
        const { ArtistId } = this.state;
        const NewArtistId = nextProps.match.params.id;

        if (ArtistId !== NewArtistId && ArtistId !== undefined) {
            this.fetchArtist(NewArtistId);
        }
    }

    fetchArtist = (artist_id) => {
        this.setState({ isFetching: true });
        Axios.get(`/api/artist/${artist_id}`).then((res) => {
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
                    title="Top Musics"
                    showMore={TopTracks.length > displayedTopTracks}
                    onMoreClick={() => {
                        this.setState({ displayedTopTracks: displayedTopTracks + 8 });
                    }}
                    isLoading={isFetching}
                />
                <AlbumGroup
                    Albums={AlbumsId.slice(0, displayedAlbums)}
                    title="Albums"
                    showMore={AlbumsId.length > displayedAlbums}
                    onMoreClick={() => {
                        this.setState({ displayedAlbums: displayedAlbums + 8 });
                    }}
                    isLoading={isFetching}
                />
                <ArtistGroup
                    Artists={RelatedArtists.slice(0, displayedRelArtists)}
                    title="Related Artists"
                    showMore={RelatedArtists.length > displayedRelArtists}
                    onMoreClick={() => {
                        this.setState({ displayedRelArtists: displayedRelArtists + 8 });
                    }}
                    isLoading={isFetching}
                />
            </>
        );
    }
}

export default Artist;
