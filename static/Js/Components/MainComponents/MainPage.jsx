import React from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import { withRouter } from 'react-router-dom';
import Favorites from './Favorites';
import History from './History';
import RelatedMusics from './RelatedMusics';
import Trending from './Trending';

const mapStateToProps = (state) => ({
    Account: state.UserAccountReducer.Account,
});

class MainPageConnected extends React.Component {
    static propTypes = {
        Account: PropTypes.shape({
            liked_musics: PropTypes.arrayOf(PropTypes.number),
        }).isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {};
    }

    render() {
        const { Account } = this.props;
        const { liked_musics } = Account;
        console.log(liked_musics);
        return (
            <>
                <RelatedMusics Title="Suggestion" Limit={10} ExcludePassedIds={false} MusicIds={liked_musics} />
                <Trending />
                <Favorites Size={14} />
                <History Size={14} />
            </>
        );
    }
}

const MainPage = connect(mapStateToProps)(MainPageConnected);

export default withRouter(MainPage);
