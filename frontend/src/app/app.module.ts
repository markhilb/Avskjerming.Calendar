import { Injectable, LOCALE_ID, NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { HttpClientModule } from '@angular/common/http';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { StoreModule } from '@ngrx/store';
import { StoreDevtoolsModule } from '@ngrx/store-devtools';
import { NgbModule } from '@ng-bootstrap/ng-bootstrap';
import {
  CalendarDateFormatter,
  CalendarModule,
  CalendarNativeDateFormatter,
  DateAdapter,
  DateFormatterParams,
} from 'angular-calendar';
import { adapterFactory } from 'angular-calendar/date-adapters/date-fns';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { FlatpickrModule } from 'angularx-flatpickr';
import { environment } from 'src/environments/environment';
import { appEffects, appReducers } from './store';
import { EffectsModule } from '@ngrx/effects';
import { ToastComponent } from './components/toast/toast.component';
import { CalendarPageComponent } from './pages/calendar-page/calendar-page.component';
import { SettingsPageComponent } from './pages/settings-page/settings-page.component';
import { LoginPageComponent } from './pages/login-page/login-page.component';
import { ChangePasswordPageComponent } from './pages/change-password-page/change-password-page.component';
import { WeekEventComponent } from './pages/calendar-page/week-event/week-event.component';
import { LetDirective, PushPipe } from '@ngrx/component';
import { registerLocaleData } from '@angular/common';
import localeNo from '@angular/common/locales/no';

@Injectable()
class CustomDateFormatter extends CalendarNativeDateFormatter {
  override dayViewHour({ date }: DateFormatterParams): string {
    return new Intl.DateTimeFormat('no-NO', {
      hour: 'numeric',
      minute: 'numeric',
    }).format(date);
  }

  override weekViewHour({ date }: DateFormatterParams): string {
    return new Intl.DateTimeFormat('no-NO', {
      hour: 'numeric',
      minute: 'numeric',
    }).format(date);
  }
}

registerLocaleData(localeNo);

@NgModule({
  declarations: [
    AppComponent,
    ToastComponent,
    CalendarPageComponent,
    SettingsPageComponent,
    LoginPageComponent,
    ChangePasswordPageComponent,
    WeekEventComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
    FormsModule,
    ReactiveFormsModule,
    NgbModule,
    LetDirective,
    PushPipe,
    FlatpickrModule.forRoot(),
    StoreModule.forRoot(appReducers),
    EffectsModule.forRoot(appEffects),
    StoreDevtoolsModule.instrument({
      maxAge: 25,
      logOnly: environment.production,
    }),
    CalendarModule.forRoot(
      {
        provide: DateAdapter,
        useFactory: adapterFactory,
      },
      {
        dateFormatter: {
          provide: CalendarDateFormatter,
          useClass: CustomDateFormatter,
        },
      },
    ),
  ],
  providers: [{ provide: LOCALE_ID, useValue: 'no-NO' }],
  bootstrap: [AppComponent],
})
export class AppModule {}
