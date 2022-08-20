import React from 'react';
import Axios from 'axios';
import { connect } from 'react-redux';
import PropTypes from 'prop-types';
import { withRouter, Link } from 'react-router-dom';
import { Button, NavDropdown } from 'react-bootstrap';

import { AddMyAccount as AddMyAccountRedux } from '../../Actions/Action';
import PlaylistImporterModal from '../Helper/PlaylistImporterModal';

function mapDispatchToProps(dispatch) {
    return {
        AddMyAccount: (Account) => dispatch(AddMyAccountRedux(Account)),
    };
}

const mapStateToProps = (state) => ({
    Account: state.UserAccountReducer.Account,
});

class AccountTopNavConnected extends React.Component {
    static propTypes = {
        Account: PropTypes.shape({
            username: PropTypes.string.isRequired,
            _id: PropTypes.string.isRequired,
        }),
        AddMyAccount: PropTypes.func.isRequired,
        history: PropTypes.shape({
            push: PropTypes.func.isRequired,
        }).isRequired,
    };

    static defaultProps = {
        Account: undefined,
    };

    constructor(props) {
        super(props);
        this.state = {
            ShowPlaylistImporterModal: false,
        };
    }

    componentDidMount() {
        this.requestMyAccount();
    }

    requestMyAccount = () => {
        const { AddMyAccount } = this.props;

        Axios.get('/User/Me')
            .then((res) => {
                if (res.data.Account) {
                    AddMyAccount(res.data.Account);
                }
            })
            .catch(() => {});
    };

    OnLogout = () => {
        const { AddMyAccount, history } = this.props;

        Axios.post('/User/Logout').then(() => {
            AddMyAccount(undefined);
            history.push('/Login');
        });
    };

    OnFavorites = () => {
        const { history } = this.props;

        history.push('/Favorites');
    };

    OnHistory = () => {
        const { history } = this.props;

        history.push('/History');
    };

    OnPlaylists = () => {
        const { history, Account } = this.props;

        history.push(`/User/${Account._id}/Playlists`);
    };

    OnImportPlaylist = () => {
        this.setState({ ShowPlaylistImporterModal: true });
    };

    OnPlaylistModalClose = () => {
        this.setState({ ShowPlaylistImporterModal: false });
    };

    render() {
        const { Account } = this.props;
        const { ShowPlaylistImporterModal } = this.state;

        if (Account) {
            const { username } = Account;
            return (
                <>
                    {ShowPlaylistImporterModal && (
                        <PlaylistImporterModal OnClose={this.OnPlaylistModalClose} />
                    )}
                    <NavDropdown title={username} id="basic-nav-dropdown">
                        <NavDropdown.Item onClick={this.OnFavorites}>Favorites</NavDropdown.Item>
                        <NavDropdown.Item onClick={this.OnHistory}>History</NavDropdown.Item>
                        <NavDropdown.Item onClick={this.OnPlaylists}>Playlists</NavDropdown.Item>
                        <NavDropdown.Divider />
                        <NavDropdown.Item onClick={this.OnImportPlaylist}>
                            Import Playlist
                        </NavDropdown.Item>
                        <NavDropdown.Divider />
                        <NavDropdown.Item onClick={this.OnLogout}>Logout</NavDropdown.Item>
                    </NavDropdown>
                </>
            );
        }
        return (
            <>
                <Link to="/Register">
                    <Button variant="primary" className="mr-2">
                        Register
                    </Button>
                </Link>
                <Link to="/Login">
                    <Button variant="outline-primary" className="mr-2">
                        Login
                    </Button>
                </Link>
            </>
        );
    }
}

const AccountTopNav = connect(mapStateToProps, mapDispatchToProps)(AccountTopNavConnected);

export default withRouter(AccountTopNav);
