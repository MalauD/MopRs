import { ADD_MY_ACCOUNT } from '../Actions/Action';

const InitialState = {
	Account: undefined,
};

export default function UserAccountReducer(state = InitialState, action) {
	switch (action.type) {
	case ADD_MY_ACCOUNT:
		return {
			...state,
			Account: action.Account,
		};
	default:
		return state;
	}
}
