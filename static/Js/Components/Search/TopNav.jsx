import React from 'react';
import {
	Navbar, FormControl, Form,
} from 'react-bootstrap';
import PropTypes from 'prop-types';
import AccountTopNav from '../Authentification/AccountTopNav';
import Icon from '../Helper/Icon';

export default class TopNav extends React.Component {
	static propTypes = {
		history: PropTypes.shape({
			push: PropTypes.func.isRequired,
		}).isRequired,
	}

	constructor(props) {
		super(props);
		this.state = {
			SearchValue: '',
		};
	}

	handleSearch = () => {
		const { history } = this.props;
		const { SearchValue } = this.state;
		history.push(`/Search?q=${SearchValue}`);
	};

	handleInputChange = (event) => {
		this.setState({
			SearchValue: event.target.value,
		});
	};

	render() {
		const { SearchValue } = this.state;
		return (
			<Navbar variant="" bg="" expand="lg" className="justify-content-between">
				<Navbar.Brand href="#">Mop - Js Edition</Navbar.Brand>
				<Navbar.Toggle aria-controls="basic-navbar-nav">
					<Icon
						dataEva="menu-2-outline"
						evaOptions={{fill: "#d6d6d6ff"}}
						style={{
							fontSize: '1.5rem',
						}}
					/>

				</Navbar.Toggle>
				<Navbar.Collapse id="basic-navbar-nav" className="justify-content-end">
					<Form inline onSubmit={this.handleSearch}>
						<FormControl type="text" placeholder="Search for musics" value={SearchValue} onChange={this.handleInputChange} className=" mr-sm-4 my-1" />
					</Form>
					<AccountTopNav />
				</Navbar.Collapse>
			</Navbar>
		);
	}
}
