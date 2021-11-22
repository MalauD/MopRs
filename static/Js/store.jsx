import { createStore } from 'redux';
import onStoreDispatch from './Reducers/StoreSubscribers';
import RootReducer from './Reducers/RootReducer';

const store = createStore(RootReducer, window.__REDUX_DEVTOOLS_EXTENSION__
        && window.__REDUX_DEVTOOLS_EXTENSION__());

store.subscribe(() => { onStoreDispatch(store); });

export default store;
