import React, { useState } from 'react';
import QueryString from 'query-string';
import { Form, Button, Row } from 'react-bootstrap';
import { useForm } from 'react-hook-form';
import Axios from 'axios';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import { Link, useLocation } from 'react-router-dom';
import { AddMyAccount } from '../../Actions/Action';

function LoginConnected({ history, dispatch }) {
    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm();
    const location = useLocation();

    const [externalError, setexternalError] = useState('');

    const values = QueryString.parse(location.search);

    const onSubmit = (data) => {
        Axios.post('/api/login', data)
            .then((res) => {
                if (res.data.success) {
                    Axios.get('/api/me')
                        .then((res2) => {
                            if (res2.data.Account) {
                                dispatch(AddMyAccount(res2.data.Account));
                            }
                            history.push(values.follow || '/');
                        })
                        .catch(() => {
                            history.push(values.follow || '/');
                        });
                } else {
                    setexternalError('Invalid account');
                }
            })
            .catch(() => {
                setexternalError('Invalid account');
            });
    };

    return (
        <Form className="m-5" onSubmit={handleSubmit(onSubmit)}>
            <h2 className="text-center">Login</h2>

            <Form.Group controlId="formBasicEmail">
                <Form.Label>Username</Form.Label>
                <Form.Control
                    name="username"
                    type="text"
                    placeholder="Enter username"
                    {...register('username', { required: true, minLength: 3, maxLength: 20 })}
                />
                <Form.Text className="text-muted">
                    {errors.username && 'Username is required and must be valid.'}
                </Form.Text>
            </Form.Group>

            <Form.Group controlId="formBasicPassword">
                <Form.Label>Password</Form.Label>
                <Form.Control
                    name="password"
                    type="password"
                    placeholder="Password"
                    {...register('password', { required: true, minLength: 8 })}
                />
                <Form.Text className="text-muted">
                    {errors.password && 'Password is required and should be at least 8 characters.'}
                </Form.Text>
            </Form.Group>

            <Form.Text className="text-muted mb-1">{externalError}</Form.Text>

            <Row>
                <div className="col-md-auto py-auto">
                    <Button variant="primary" type="submit">
                        Login
                    </Button>
                </div>
                <div className="col-md-auto">
                    <Link className="my-auto" to={`/Register?follow=${values.follow || '/'}`}>
                        Create an account now
                    </Link>
                </div>
            </Row>
        </Form>
    );
}

LoginConnected.propTypes = {
    history: PropTypes.shape({
        push: PropTypes.func.isRequired,
    }).isRequired,
    dispatch: PropTypes.func.isRequired,
};

const Login = connect()(LoginConnected);

export default Login;
