import { inject } from '@angular/core';
import { CanActivateFn, Router } from '@angular/router';
import { Store } from '@ngrx/store';
import { AppState, selectLoggedInDefined } from '../store';
import { map } from 'rxjs';
import { environment } from 'src/environments/environment';

export const authGuard: CanActivateFn = (_route, _state) => {
  const router = inject(Router);

  if (!environment.auth) {
    return true;
  }

  const store = inject(Store<AppState>);

  return store.pipe(
    selectLoggedInDefined,
    map((loggedIn) => (loggedIn ? true : router.parseUrl('login'))),
  );
};

export const anonGuard: CanActivateFn = (_route, _state) => {
  const router = inject(Router);

  if (!environment.auth) {
    return router.parseUrl('');
  }

  const store = inject(Store<AppState>);

  return store.pipe(
    selectLoggedInDefined,
    map((loggedIn) => (loggedIn ? router.parseUrl('') : true)),
  );
};
