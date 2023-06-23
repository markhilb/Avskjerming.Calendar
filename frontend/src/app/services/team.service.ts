import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { environment } from 'src/environments/environment';
import { TeamService as BaseTeamService, Configuration, CreateTeam, Team } from 'src/generated/openapi';

@Injectable({
  providedIn: 'root',
})
export class TeamService {
  api: BaseTeamService;

  constructor(httpClient: HttpClient) {
    this.api = new BaseTeamService(httpClient, environment.baseUrl, new Configuration({ withCredentials: true }));
  }

  getTeams = () => this.api.getTeams();

  createTeam = (team: CreateTeam) => this.api.createTeam(team);

  updateTeam = (team: Team) => this.api.updateTeam(team);

  deleteTeam = (id: number) => this.api.deleteTeam(id);
}
