import React from 'react';
import PlaylistContainer from './../Containers/PlaylistContainer';
import { withRouter } from 'react-router-dom';

class PlayerFull extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }

    render() {
        return (
            <>
                <PlaylistContainer />
            </>
        );
    }
}

export default withRouter(PlayerFull);
