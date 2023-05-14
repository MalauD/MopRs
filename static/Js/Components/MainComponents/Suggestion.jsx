import React from 'react';
import Axios from 'axios';
import MusicGroup from './Groups/MusicGroup';

function Suggestion() {
    const [isFetching, setIsFetching] = React.useState(false);
    const [suggestions, setSuggestions] = React.useState([]);

    React.useEffect(() => {
        setIsFetching(true);
        Axios.get('/api/me/suggestions?memory=100&likeHistRatio=0.25&novelty=0.2&limit=15').then(
            (res) => {
                setSuggestions(res.data);
                setIsFetching(false);
            }
        );
    }, []);

    return <MusicGroup Musics={suggestions} title="Suggestions" isLoading={isFetching} />;
}

export default Suggestion;
