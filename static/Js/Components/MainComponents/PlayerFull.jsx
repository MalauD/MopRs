import React from 'react';
import { withRouter } from 'react-router-dom';
import PlaylistContainer from "../Containers/PlaylistContainer";

class PlayerFull extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }

    render() {
        return (
            <PlaylistContainer />
        );
    }
}

export default withRouter(PlayerFull);
