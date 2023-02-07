import React from 'react';
import { withRouter } from 'react-router-dom';
import PropTypes from 'prop-types';
import { Dropdown } from 'react-bootstrap';

class GetLink extends React.Component {
    static propTypes = {
        Music: PropTypes.shape({
            _id: PropTypes.number.isRequired,
        }).isRequired,
        history: PropTypes.shape({
            push: PropTypes.func.isRequired,
        }).isRequired,
    };

    onClick = () => {
        const { Music, history } = this.props;
        history.push(`/Music/${Music._id}`);
    };

    render() {
        return <Dropdown.Item onClick={this.onClick}>Get link</Dropdown.Item>;
    }
}

export default withRouter(GetLink);
