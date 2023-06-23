import { Team } from 'src/generated/openapi';

export interface TeamState {
  teams: Team[];
}

export const initialTeamState: TeamState = {
  teams: [],
};
