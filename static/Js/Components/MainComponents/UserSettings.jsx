import Axios from 'axios';
import React from 'react';

export default function UserSettings() {
    const [preferedFormat, setPreferedFormat] = React.useState('MP3_128');
    const [isFetching, setIsFetching] = React.useState(true);

    React.useEffect(() => {
        Axios.get('/api/me/preferedformat')
            .then((res) => {
                setPreferedFormat(res.data);
                setIsFetching(false);
            })
            .catch(() => {});
    }, []);

    if (isFetching) {
        return null;
    }

    return (
        <>
            <h2
                style={{
                    fontFamily: 'Pacifico, cursive',
                    textShadow: '#cc506c 3px 3px 0px',
                    marginBottom: '1em',
                }}
                className="align-self-center text-center"
            >
                Settings
            </h2>
            <div className="d-flex flex-column align-items-center">
                <div className="d-flex flex-column align-items-center">
                    <h5>Prefered format</h5>
                    <select
                        className="form-control"
                        value={preferedFormat}
                        onChange={(e) => {
                            setPreferedFormat(e.target.value);
                            Axios.post('/api/me/preferedformat', { format: e.target.value });
                        }}
                    >
                        <option value="MP3_128">MP3 128kbps</option>
                        <option value="MP3_320">MP3 320kbps</option>
                        <option value="FLAC">FLAC</option>
                    </select>
                </div>
            </div>
        </>
    );
}
