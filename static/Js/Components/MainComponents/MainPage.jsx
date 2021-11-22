import React from 'react';
import Favorites from './Favorites';
import History from './History';
import Selection from './Selection';
import Trending from './Trending';
import { withRouter } from 'react-router-dom';

class MainPage extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }

    render() {
        return (
            <>
                <Selection />
                <Trending />
                <Favorites Size={10} />
                <History Size={10} />
            </>
        );
    }
}

export default withRouter(MainPage);
