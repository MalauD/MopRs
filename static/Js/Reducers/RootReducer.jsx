import { combineReducers } from 'redux';
import MusicPlayerReducer from './MusicPlayerReducer';
import UserAccountReducer from './UserAccountReducer';

export default combineReducers({
	MusicPlayerReducer,
	UserAccountReducer,
});
