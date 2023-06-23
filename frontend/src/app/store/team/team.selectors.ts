import { createSelector } from '@ngrx/store';
import { AppState } from '../app.state';

const teamState = (state: AppState) => state.teamState;

export const selectTeams = createSelector(teamState, (state) => state.teams);

export const selectTeamsMap = createSelector(selectTeams, (state) => Object.fromEntries(state.map((t) => [t.id, t])));
