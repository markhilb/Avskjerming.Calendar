import { createReducer, on } from '@ngrx/store';
import { initialAuthState } from './auth.state';
import * as A from './auth.actions';

export const authReducer = createReducer(
  initialAuthState,
  on(A.loginOk, (state) => ({ ...state, loggedIn: true })),
  on(A.loggedInOk, (state, { loggedIn }) => ({ ...state, loggedIn })),
);
