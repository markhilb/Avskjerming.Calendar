import { Component } from '@angular/core';
import { Store } from '@ngrx/store';
import { AppState, loggedIn } from './store';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent {
  constructor(store: Store<AppState>) {
    store.dispatch(loggedIn());
  }
}
