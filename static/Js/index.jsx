import Axios from 'axios';
import React, { Suspense } from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import { HashRouter, Route } from 'react-router-dom';

import '../Style/Main.scss';

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
import Music from './Components/MainComponents/Music';
import Album from './Components/MainComponents/Album';
import Artist from './Components/MainComponents/Artist';
import UserPlaylist from './Components/MainComponents/UserPlaylist';
import UserPlaylists from './Components/MainComponents/UserPlaylists';
import PlayerFull from './Components/MainComponents/PlayerFull';
import UserSettings from './Components/MainComponents/UserSettings';

Axios.defaults.withCredentials = true;

function App() {
    return (
        <Provider store={store}>
            <Suspense fallback={<div>Loading...</div>}>
                <HashRouter>
                    <ScrollToTop />
                    <Route path="/">
                        <ProtectedRoute Component={TopNav} />
                    </Route>
                    <Route exact path="/">
                        <ProtectedRoute Component={MainPage} />
                    </Route>
                    <Route path="/" component={Player} />
                    <Route path="/Login" component={Login} />
                    <Route path="/Register" component={Register} />
                    <Route path="/Favorites">
                        <ProtectedRoute Component={Favorites} />
                    </Route>
                    <Route path="/History">
                        <ProtectedRoute Component={History} />
                    </Route>
                    <Route path="/Search">
                        <ProtectedRoute Component={SearchPage} />
                    </Route>
                    <Route path="/Player">
                        <ProtectedRoute Component={PlayerFull} />
                    </Route>
                    <Route path="/Music/:id">
                        <ProtectedRoute Component={Music} />
                    </Route>
                    <Route path="/Album/:id">
                        <ProtectedRoute Component={Album} />
                    </Route>
                    <Route path="/Artist/:id">
                        <ProtectedRoute Component={Artist} />
                    </Route>
                    <Route path="/Playlist/:id">
                        <ProtectedRoute Component={UserPlaylist} />
                    </Route>
                    <Route path="/User/:id/Playlists">
                        <ProtectedRoute Component={UserPlaylists} />
                    </Route>
                    <Route path="/Settings">
                        <ProtectedRoute Component={UserSettings} />
                    </Route>
                </HashRouter>
            </Suspense>
        </Provider>
    );
}

ReactDOM.render(<App />, document.querySelector('#root'));
