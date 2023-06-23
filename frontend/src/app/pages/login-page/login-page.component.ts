import { Component } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { ofType } from '@ngrx/effects';
import { ActionsSubject, Store } from '@ngrx/store';
import { AppState, login, loginFailed } from 'src/app/store';
import { Login } from 'src/generated/openapi';

@UntilDestroy()
@Component({
  selector: 'app-login-page',
  templateUrl: './login-page.component.html',
  styleUrls: ['./login-page.component.scss'],
})
export class LoginPageComponent {
  loginFailed = false;

  fg = new FormGroup({
    password: new FormControl('', [Validators.required]),
  });

  constructor(private store: Store<AppState>, actions: ActionsSubject) {
    setTimeout(() => document.getElementById('password')?.focus(), 0);

    actions.pipe(untilDestroyed(this), ofType(loginFailed)).subscribe(() => (this.loginFailed = true));
  }

  login() {
    console.log(this.fg.value, this.fg.valid);
    if (this.fg.valid) {
      this.store.dispatch(login({ login: this.fg.value as Login }));
    } else {
      this.fg.markAllAsTouched();
    }
  }
}
