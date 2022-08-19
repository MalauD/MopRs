import React from 'react';
import { Dropdown } from 'react-bootstrap';
import PlayMusic from './PlayMusic';
import AddMusic from './AddMusic';
import AddToNewPlaylistMusic from './AddToNewPlaylistMusic';
import PlayNextMusic from './PlayNextMusic';
import AddToPlaylistMusic from './AddToPlaylistMusic';
import DeletePlaylist from './DeletePlaylistMusic';
import AddPlaylistMusic from './AddPlaylistMusic';

function DefaultActions(props) {
    return (
        <>
            <PlayMusic {...props} />
            <PlayNextMusic {...props} />
            <Dropdown.Divider />
            <AddMusic {...props} />
            <Dropdown.Divider />
            <AddToNewPlaylistMusic {...props} />
            <AddToPlaylistMusic {...props} />
        </>
    );
}

function OwnPlaylistActions(props) {
    return (
        <>
            <PlayMusic {...props} />
            <PlayNextMusic {...props} />
            <Dropdown.Divider />
            <AddMusic {...props} />
            <Dropdown.Divider />
            <AddToNewPlaylistMusic {...props} />
            <AddToPlaylistMusic {...props} />
            <Dropdown.Divider />
            <DeletePlaylist {...props} />
        </>
    );
}

function OwnPlaylistRelatedActions(props) {
    return (
        <>
            <AddPlaylistMusic {...props} />
            <Dropdown.Divider />
            <PlayMusic {...props} />
            <PlayNextMusic {...props} />
            <Dropdown.Divider />
            <AddMusic {...props} />
            <Dropdown.Divider />
            <AddToNewPlaylistMusic {...props} />
            <AddToPlaylistMusic {...props} />
            <Dropdown.Divider />
            <DeletePlaylist {...props} />
        </>
    );
}

export { DefaultActions, OwnPlaylistActions, OwnPlaylistRelatedActions };
