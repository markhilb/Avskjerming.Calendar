import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { EMPTY } from 'rxjs';
import { catchError, exhaustMap, map, take, tap } from 'rxjs/operators';
import { AuthService } from 'src/app/services/auth.service';
import * as A from './auth.actions';
import { ToastService } from 'src/app/services/toast.service';

@Injectable()
export class AuthEffects {
  login$ = createEffect(() =>
    this.actions.pipe(
      ofType(A.login),
      exhaustMap((action) =>
        this.authService.login(action.login).pipe(
          map((loggedIn) => {
            if (loggedIn) {
              location.assign('');
              return A.loginOk();
            }
            return A.loginFailed();
          }),
          catchError(() => EMPTY),
        ),
      ),
    ),
  );

  logout$ = createEffect(() =>
    this.actions.pipe(
      ofType(A.logout),
      exhaustMap(() =>
        this.authService.logout().pipe(
          map(() => A.logoutOk()),
          catchError(() => EMPTY),
        ),
      ),
    ),
  );

  logoutOk$ = createEffect(() =>
    this.actions.pipe(
      ofType(A.logoutOk),
      take(1),
      tap(() => location.assign('')),
    ),
  );

  loggedIn$ = createEffect(() =>
    this.actions.pipe(
      ofType(A.loggedIn),
      exhaustMap(() =>
        this.authService.loggedIn().pipe(
          map((loggedIn) => A.loggedInOk({ loggedIn })),
          catchError(() => EMPTY),
        ),
      ),
    ),
  );

  changePassword$ = createEffect(() =>
    this.actions.pipe(
      ofType(A.changePassword),
      exhaustMap((action) =>
        this.authService.changePassword(action.changePassword).pipe(
          map((success) => {
            if (success) {
              this.toastService.success('Passord endret');
              return A.changePasswordOk();
            }
            return A.changePasswordFailed();
          }),
          catchError(() => EMPTY),
        ),
      ),
    ),
  );

  constructor(private actions: Actions, private authService: AuthService, private toastService: ToastService) {}
}
