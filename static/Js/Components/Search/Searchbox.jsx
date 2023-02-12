import React, { useEffect } from 'react';
import { useHistory, useLocation } from 'react-router-dom';
import Turnstone from 'turnstone';
import PropTypes from 'prop-types';
import MusicItemRow from '../Items/MusicItemRow';

function SearchGroupName(props) {
    const { children } = props;
    return <></>;
}

function SearchMusicItem(props) {
    const { item } = props;

    return (
        <MusicItemRow
            ImageDz={item.image_url}
            Title={item.title}
            Artist={item.artist_name}
            UseDragHandle={false}
            Highlight={false}
        />
    );
}

SearchMusicItem.propTypes = {
    item: PropTypes.shape({
        _id: PropTypes.number.isRequired,
        image_url: PropTypes.string,
        title: PropTypes.string.isRequired,
        artist_name: PropTypes.string.isRequired,
    }).isRequired,
};

const styles = {
    input: 'SearchboxInput',
    inputFocus: 'SearchboxInputFocus',
    query: '',
    typeahead: 'SearchboxTypeahead',
    cancelButton: ``,
    clearButton: '',
    listbox: 'SearchboxItemList',
    groupHeading: '',
    item: '',
    highlightedItem: '',
};

export default function Searchbox() {
    const history = useHistory();
    const [lastSelectedId, setlastSelectedId] = React.useState(null);
    const searchboxRef = React.useRef(null);

    const listbox = [
        {
            id: 'musics',
            name: 'Musics',
            ratio: 4,
            displayField: 'title',
            data: (query) =>
                fetch(
                    `/api/search/music/${encodeURIComponent(
                        query
                    )}?maxResults=14&page=0&no_index=true`
                ).then((response) => response.json()),
            searchType: 'contains',
        },
    ];

    const onSelect = (item) => {
        if (item && item.id !== lastSelectedId) {
            setlastSelectedId(item.id);
            history.push(`/Music/${item.id}`);
        }
    };

    const onEnter = (query) => {
        setlastSelectedId(lastSelectedId < 0 ? lastSelectedId - 1 : -1);
        if (query) {
            history.push(`/Search?q=${encodeURIComponent(query)}`);
        }
    };

    return (
        <Turnstone
            enterKeyHint="search"
            debounceWait={250}
            key={lastSelectedId}
            id="searchbox"
            listbox={listbox}
            listboxIsImmutable
            maxItems={5}
            styles={styles}
            Item={SearchMusicItem}
            GroupName={SearchGroupName}
            onSelect={onSelect}
            onEnter={onEnter}
            ref={searchboxRef}
            placeholder="Search for musics ..."
        />
    );
}
