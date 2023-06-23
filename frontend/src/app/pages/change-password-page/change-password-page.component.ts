import { Component } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { ofType } from '@ngrx/effects';
import { ActionsSubject, Store } from '@ngrx/store';
import { AppState, changePassword, changePasswordFailed, changePasswordOk } from 'src/app/store';
import { ChangePassword } from 'src/generated/openapi';

@UntilDestroy()
@Component({
  selector: 'app-change-password-page',
  templateUrl: './change-password-page.component.html',
  styleUrls: ['./change-password-page.component.scss'],
})
export class ChangePasswordPageComponent {
  failed = false;

  fg = new FormGroup({
    oldPassword: new FormControl('', [Validators.required]),
    newPassword: new FormControl('', [Validators.required]),
  });

  constructor(private store: Store<AppState>, actions: ActionsSubject, router: Router) {
    setTimeout(() => document.getElementById('oldPassword')?.focus(), 0);

    actions.pipe(untilDestroyed(this), ofType(changePasswordFailed)).subscribe(() => (this.failed = true));
    actions.pipe(untilDestroyed(this), ofType(changePasswordOk)).subscribe(() => router.navigateByUrl(''));
  }

  submit() {
    if (this.fg.valid) {
      this.store.dispatch(changePassword({ changePassword: this.fg.value as ChangePassword }));
    } else {
      this.fg.markAllAsTouched();
    }
  }
}
