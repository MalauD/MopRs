import React from 'react';
import SearchPageAlbums from './SearchPageAlbums';
import SearchPageArtists from './SearchPageArtists';
import SearchPageMusics from './SearchPageMusics';
import SearchPagePlaylists from './SearchPagePlaylists';

const SearchPage = (props) => (
    <>
        <SearchPageMusics {...props} />
        <SearchPageAlbums {...props} />
        <SearchPageArtists {...props} />
        <SearchPagePlaylists {...props} />
    </>
);

export default SearchPage;
