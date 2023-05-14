import React from 'react';
import { withRouter } from 'react-router-dom';
import Favorites from './Favorites';
import History from './History';
import Trending from './Trending';
import Suggestion from './Suggestion';

class MainPage extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }

    render() {
        return (
            <>
                <Suggestion />
                <Trending />
                <Favorites Size={14} />
                <History Size={14} />
            </>
        );
    }
}

export default withRouter(MainPage);
