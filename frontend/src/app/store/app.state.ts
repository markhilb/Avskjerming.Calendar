import { AuthState, EventState, initialAuthState, initialEventState } from '.';
import { EmployeeState, initialEmployeeState } from './employee';
import { initialTeamState, TeamState } from './team';

export interface AppState {
  eventState: EventState;
  teamState: TeamState;
  employeeState: EmployeeState;
  authState: AuthState;
}

export const initialAppState: AppState = {
  eventState: initialEventState,
  teamState: initialTeamState,
  employeeState: initialEmployeeState,
  authState: initialAuthState,
};
