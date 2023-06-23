import { createAction, props } from '@ngrx/store';
import { CreateTeam, Team } from 'src/generated/openapi';

export const getTeams = createAction('[Team] Get teams');
export const getTeamsOk = createAction('[Team] Get teams Ok', props<{ teams: Team[] }>());

export const createTeam = createAction('[Team] Create team', props<{ team: CreateTeam }>());
export const createTeamOk = createAction('[Team] Create team Ok', props<{ team: Team }>());

export const updateTeam = createAction('[Team] Update team', props<{ team: Team }>());
export const updateTeamOk = createAction('[Team] Update team Ok', props<{ team: Team }>());

export const deleteTeam = createAction('[Team] Delete team', props<{ id: number }>());
export const deleteTeamOk = createAction('[Team] Delete team Ok', props<{ id: number }>());
