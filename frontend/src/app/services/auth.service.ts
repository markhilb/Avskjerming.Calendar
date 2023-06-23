import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { map } from 'rxjs';
import { environment } from 'src/environments/environment';
import { AuthService as BaseAuthService, Configuration, Login, ChangePassword } from 'src/generated/openapi';

// Stupid bug where a body constaining `true` or `false` is parsed as a string
const stringToBoolean = (string: any) => string.toString() === 'true';

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  api: BaseAuthService;

  constructor(httpClient: HttpClient) {
    this.api = new BaseAuthService(httpClient, environment.baseUrl, new Configuration({ withCredentials: true }));
  }

  login = (login: Login) => this.api.login(login).pipe(map(stringToBoolean));

  logout = () => this.api.logout();

  loggedIn = () => this.api.loggedIn().pipe(map(stringToBoolean));

  changePassword = (changePassword: ChangePassword) =>
    this.api.changePassword(changePassword).pipe(map(stringToBoolean));
}
