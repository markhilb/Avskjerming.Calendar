import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { anonGuard, authGuard } from './guards/auth.guard';
import { CalendarPageComponent } from './pages/calendar-page/calendar-page.component';
import { ChangePasswordPageComponent } from './pages/change-password-page/change-password-page.component';
import { LoginPageComponent } from './pages/login-page/login-page.component';
import { SettingsPageComponent } from './pages/settings-page/settings-page.component';

const routes: Routes = [
  {
    path: '',
    component: CalendarPageComponent,
    canActivate: [authGuard],
  },
  {
    path: 'login',
    component: LoginPageComponent,
    canActivate: [anonGuard],
  },
  {
    path: 'instillinger',
    component: SettingsPageComponent,
    canActivate: [authGuard],
  },
  {
    path: 'passord',
    component: ChangePasswordPageComponent,
    canActivate: [authGuard],
  },

  // Must be at the bottom
  {
    path: '**',
    redirectTo: '',
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
