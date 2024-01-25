import React from 'react';
import { Navbar } from 'react-bootstrap';
import { useHistory } from 'react-router-dom';
import AccountTopNav from '../Authentification/AccountTopNav';
import Icon from '../Helper/Icon';
import Searchbox from './Searchbox';

export default function TopNav() {
    const history = useHistory();

    const onLogoClick = () => {
        history.push('/');
    };

    return (
        <Navbar variant="" bg="" expand="lg">
            <Navbar.Brand
                style={{
                    fontFamily: 'Pacifico, cursive',
                    fontSize: '2em',
                    marginLeft: '0.4em',
                    textShadow: '#cc506c 3px 3px 0px',
                }}
                onClick={onLogoClick}
            >
                MOP
            </Navbar.Brand>
            <Navbar.Toggle aria-controls="basic-navbar-nav">
                <Icon
                    dataEva="menu-2-outline"
                    evaOptions={{ fill: '#d6d6d6ff' }}
                    style={{
                        fontSize: '1.5rem',
                    }}
                />
            </Navbar.Toggle>

            <Navbar.Collapse className="justify-content-end">
                <Searchbox />
            </Navbar.Collapse>
            <Navbar.Collapse className="justify-content-end">
                <AccountTopNav />
            </Navbar.Collapse>
        </Navbar>
    );
}
