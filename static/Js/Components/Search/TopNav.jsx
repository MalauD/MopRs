import React from 'react';
import { Navbar } from 'react-bootstrap';
import AccountTopNav from '../Authentification/AccountTopNav';
import Icon from '../Helper/Icon';
import Searchbox from './Searchbox';

export default class TopNav extends React.Component {
    render() {
        return (
            <Navbar variant="" bg="" expand="lg" className="justify-content-between">
                <Navbar.Brand
                    style={{
                        fontFamily: 'Pacifico, cursive',
                        fontSize: '2em',
                        marginLeft: '0.4em',
                        textShadow: '#cc506c 3px 3px 0px',
                    }}
                    href="#"
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
                <Navbar.Collapse id="basic-navbar-nav" className="justify-content-end">
                    <Searchbox />
                    <AccountTopNav />
                </Navbar.Collapse>
            </Navbar>
        );
    }
}
