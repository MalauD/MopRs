import Axios from 'axios';
import React, { Suspense } from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import { HashRouter, Route } from 'react-router-dom';
import ScrollToTop from './Components/ScrollToTop';
import ProtectedRoute from './ProtectedRoute';
import store from './store';

import TopNav from './Components/Search/TopNav';
import MainPage from './Components/MainComponents/MainPage';
import Player from './Components/MusicPlayer/Player';
import Login from './Components/Authentification/Login';
import Register from './Components/Authentification/Register';
import Favorites from './Components/MainComponents/Favorites';
import History from './Components/MainComponents/History';
import SearchPage from './Components/Search/SearchPage';
import Album from './Components/MainComponents/Album';
import Artist from './Components/MainComponents/Artist';
import UserPlaylist from './Components/MainComponents/UserPlaylist';
import UserPlaylists from './Components/MainComponents/UserPlaylists';
import PlayerFull from './Components/MainComponents/PlayerFull';

Axios.defaults.withCredentials = true;

const App = () => (
    <Provider store={store}>
        <Suspense fallback={<div>Loading...</div>}>
            <HashRouter>
                <ScrollToTop />
                <ProtectedRoute path="/" component={TopNav} />
                <ProtectedRoute exact path="/" component={MainPage} />
                <Route path="/" component={Player} />
                <Route path="/Login" component={Login} />
                <Route path="/Register" component={Register} />
                <ProtectedRoute path="/Favorites" component={Favorites} />
                <ProtectedRoute path="/History" component={History} />
                <ProtectedRoute path="/Search" component={SearchPage} />
                <ProtectedRoute path="/Player" component={PlayerFull} />
                <ProtectedRoute path="/Album/:id" component={Album} />
                <ProtectedRoute path="/Artist/:id" component={Artist} />
                <ProtectedRoute path="/Playlist/:id" component={UserPlaylist} />
                <ProtectedRoute path="/User/:id/Playlists" component={UserPlaylists} />
            </HashRouter>
        </Suspense>
    </Provider>
);

ReactDOM.render(<App />, document.querySelector('#root'));
