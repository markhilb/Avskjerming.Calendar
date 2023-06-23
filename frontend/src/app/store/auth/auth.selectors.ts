import { createSelector, select } from '@ngrx/store';
import { AppState } from '../app.state';
import { OperatorFunction, filter, pipe } from 'rxjs';

const authState = (state: AppState) => state.authState;

export const selectLoggedIn = createSelector(authState, (state) => state.loggedIn);

export const selectLoggedInDefined = pipe(
  select(selectLoggedIn),
  filter((loggedIn) => loggedIn !== undefined) as OperatorFunction<boolean | undefined, boolean>,
);
