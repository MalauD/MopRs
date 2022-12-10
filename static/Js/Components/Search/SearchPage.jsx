import React from 'react';
import SearchPageAlbums from './SearchPageAlbums';
import SearchPageArtists from './SearchPageArtists';
import SearchPageMusics from './SearchPageMusics';
import SearchPagePlaylists from './SearchPagePlaylists';

function SearchPage(props) {
    const searchAlbumsRef = React.useRef();
    const searchArtistsRef = React.useRef();

    return (
        <>
            <SearchPageMusics
                onSearchEnd={() => {
                    searchAlbumsRef.current.SearchAlbums();
                    searchArtistsRef.current.SearchArtists();
                }}
                {...props}
            />
            <SearchPageAlbums ref={searchAlbumsRef} {...props} />
            <SearchPageArtists ref={searchArtistsRef} {...props} />
            <SearchPagePlaylists {...props} />
        </>
    );
}

export default SearchPage;
