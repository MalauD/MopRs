import React, { useEffect, useState } from 'react';
import Axios from 'axios';
import MusicGroup from './Groups/MusicGroup';
import ButtonIcon from '../Helper/ButtonIcon';

const GetSuggestion = (memory, novelty, likeHistRatio, limit) =>
    Axios.get(
        `/api/me/suggestions?memory=${memory}&likeHistRatio=${likeHistRatio}&novelty=${novelty}&limit=${limit}`
    ).then((res) => res.data);

const PAGE_SIZE = 8;
const MEMORY_MULTIPLIER = 3;

function Suggestion() {
    const [isFetching, setIsFetching] = React.useState(false);
    const [suggestions, setSuggestions] = React.useState([]);
    const [page, setPage] = useState(0);

    const addSuggestion = () => {
        if (suggestions.length > 100) return;
        GetSuggestion((page + 1) * PAGE_SIZE * MEMORY_MULTIPLIER, 0.1, 0.1, PAGE_SIZE)
            .then((ApiResult) => {
                setSuggestions([...suggestions, ...ApiResult]);
                setPage(page + 1);
            })
            .catch(() => {});
    };

    const getSuggestion = () => {
        setIsFetching(true);
        setPage(0);
        GetSuggestion(PAGE_SIZE * MEMORY_MULTIPLIER, 0.1, 0.1, PAGE_SIZE)
            .then((ApiResult) => {
                setSuggestions(ApiResult);
                setIsFetching(false);
                setPage(1);
            })
            .catch(() => {
                setIsFetching(false);
            });
    };

    const Accessories = [
        <ButtonIcon
            dataEva="flip-outline"
            onClick={getSuggestion}
            evaOptions={{
                fill: '#d6d6d6ff',
                width: '30px',
                height: '30px',
            }}
        />,
    ];

    useEffect(() => {
        getSuggestion();
    }, []);

    return (
        <MusicGroup
            Musics={suggestions}
            title="Suggestions"
            isLoading={isFetching}
            showMore={suggestions.length < 100}
            Accessories={Accessories}
            onMoreClick={addSuggestion}
        />
    );
}

export default Suggestion;
