import React, { useEffect } from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import Axios from 'axios';
import { useHistory, withRouter } from 'react-router-dom';
import { AddMyAccount as AddMyAccountRedux } from './Actions/Action';

function mapDispatchToProps(dispatch) {
    return {
        AddMyAccount: (Account) => dispatch(AddMyAccountRedux(Account)),
    };
}

const mapStateToProps = (state) => ({
    IsConnected: state.UserAccountReducer?.Account !== undefined,
});

const isInAuthentification = (location) =>
    location.pathname.startsWith('/Login') || location.pathname.startsWith('/Register');

function ProtectedRouteConnected({ Component, IsConnected, AddMyAccount, ...props }) {
    const history = useHistory();
    useEffect(() => {
        if (!IsConnected) {
            Axios.get('/api/me')
                .then((res) => {
                    if (res.data.Account) {
                        AddMyAccount(res.data.Account);
                    }
                })
                .catch(() => {
                    if (!isInAuthentification(history.location)) {
                        if (history.location.pathname === '/') {
                            history.push('/Login');
                        } else {
                            history.push(`/Login?follow=${history.location.pathname}`);
                        }
                    }
                });
        }
    }, []);

    const BoundComponent = withRouter(Component);

    return IsConnected ? <BoundComponent {...props} /> : null;
}

ProtectedRouteConnected.propTypes = {
    IsConnected: PropTypes.bool.isRequired,
    Component: PropTypes.oneOfType([PropTypes.element, PropTypes.func]).isRequired,
    AddMyAccount: PropTypes.func.isRequired,
};

export default connect(mapStateToProps, mapDispatchToProps)(ProtectedRouteConnected);
