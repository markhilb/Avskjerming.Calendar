import { createReducer, on } from '@ngrx/store';
import { initialTeamState } from './team.state';
import * as A from './team.actions';

export const teamReducer = createReducer(
  initialTeamState,
  on(A.getTeamsOk, (state, { teams }) => ({ ...state, teams })),
  on(A.createTeamOk, (state, { team }) => ({ ...state, teams: [...(state?.teams ?? []), team] })),
  on(A.updateTeamOk, (state, { team }) => ({
    ...state,
    teams: state.teams?.map((x) => (x.id === team.id ? team : x)),
  })),
  on(A.deleteTeamOk, (state, { id }) => ({ ...state, teams: state.teams?.filter((x) => x.id !== id) })),
);
