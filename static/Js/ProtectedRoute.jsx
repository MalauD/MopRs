import React from 'react';
import PropTypes from 'prop-types';
import { Redirect, Route } from 'react-router-dom';
import { connect } from 'react-redux';
import Axios from 'axios';
import { AddMyAccount as AddMyAccountRedux } from './Actions/Action';

function mapDispatchToProps(dispatch) {
    return {
        AddMyAccount: (Account) => dispatch(AddMyAccountRedux(Account)),
    };
}

const mapStateToProps = (state) => ({
    Account: state.UserAccountReducer.Account,
});

class ProtectedRouteConnected extends React.Component {
    static propTypes = {
        AddMyAccount: PropTypes.func.isRequired,
        component: PropTypes.elementType.isRequired,
        Account: PropTypes.shape(),
    };

    static defaultProps = {
        Account: undefined,
    };

    constructor(props) {
        super(props);
        this.state = {
            gotResult: false,
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
                this.setState({ gotResult: true });
            })
            .catch(() => this.setState({ gotResult: true }));
    };

    render() {
        const { component: Component, Account, ...props } = this.props;
        const { gotResult } = this.state;

        return (
            <Route
                {...props}
                render={(props) =>
                    Account ? (
                        <Component {...props} />
                    ) : gotResult ? (
                        <Redirect to="/Login" />
                    ) : (
                        <div />
                    )
                }
            />
        );
    }
}

const ProtectedRoute = connect(mapStateToProps, mapDispatchToProps)(ProtectedRouteConnected);

export default ProtectedRoute;
