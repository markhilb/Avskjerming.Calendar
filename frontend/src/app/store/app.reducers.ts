import { ActionReducerMap } from '@ngrx/store';
import { AppState } from './app.state';
import { authReducer } from './auth/auth.reducers';
import { employeeReducer } from './employee/employee.reducers';
import { eventReducer } from './event/event.reducers';
import { teamReducer } from './team/team.reducers';

export const appReducers: ActionReducerMap<AppState, any> = {
  eventState: eventReducer,
  teamState: teamReducer,
  employeeState: employeeReducer,
  authState: authReducer,
};
