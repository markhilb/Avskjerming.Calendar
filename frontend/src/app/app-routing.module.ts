import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { CalendarPageComponent } from './pages/calendar-page/calendar-page.component';
import { SettingsPageComponent } from './pages/settings-page/settings-page.component';

const routes: Routes = [
  {
    path: '',
    component: CalendarPageComponent,
    // canActivate: [AuthenticationGuard],
  },
  // {
  //   path: 'login',
  //   component: LoginPageComponent,
  //   canActivate: [AnonymousGuard],
  // },
  {
    path: 'instillinger',
    component: SettingsPageComponent,
    // canActivate: [AuthenticationGuard],
  },
  // {
  //   path: 'passord',
  //   component: ChangePasswordComponent,
  //   canActivate: [AuthenticationGuard],
  // },

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
