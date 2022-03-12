import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { BaseApiService } from './base-api.service';

@Injectable({
  providedIn: 'root',
})
export class AuthenticationService {
  constructor(private api: BaseApiService) {}

  login = (password: string): Observable<boolean> => this.api.post<boolean>('login', { password });

  logout = (): Observable<void> => this.api.post<void>('logout');

  isLoggedIn = (): Observable<boolean> => this.api.get<boolean>('logged_in');

  changePassword = (oldPassword: string, newPassword: string): Observable<boolean> =>
    this.api.post<boolean>('change_password', { old: oldPassword, new: newPassword });
}
