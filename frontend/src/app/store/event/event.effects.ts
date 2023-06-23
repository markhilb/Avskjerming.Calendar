import { Injectable } from '@angular/core';
import { Actions, concatLatestFrom, createEffect, ofType } from '@ngrx/effects';
import { EMPTY } from 'rxjs';
import { catchError, exhaustMap, map, switchMap } from 'rxjs/operators';
import { EventService } from 'src/app/services/event.service';
import * as A from './event.actions';
import { selectEmployeesMap } from '../employee';
import { Store } from '@ngrx/store';
import { AppState } from '../app.state';
import { selectTeamsMap } from '../team';

@Injectable()
export class EventEffects {
  getEvents$ = createEffect(() =>
    this.actions.pipe(
      ofType(A.getEvents),
      switchMap((action) =>
        this.eventService.getEvents(action.start, action.end).pipe(
          map((events) => A.getEventsOk({ events })),
          catchError(() => EMPTY),
        ),
      ),
    ),
  );

  createEvent$ = createEffect(() =>
    this.actions.pipe(
      ofType(A.createEvent),
      exhaustMap((action) =>
        this.eventService.createEvent(action.event).pipe(
          concatLatestFrom(() => this.store.select(selectEmployeesMap)),
          concatLatestFrom(() => this.store.select(selectTeamsMap)),
          map(([[id, employees], teams]) =>
            A.createEventOk({
              event: {
                ...action.event,
                id,
                team: action.event.teamId ? teams[action.event.teamId] : undefined,
                employees: action.event.employeeIds.map((id) => employees[id]),
              },
            }),
          ),
          catchError(() => EMPTY),
        ),
      ),
    ),
  );

  updateEvent$ = createEffect(() =>
    this.actions.pipe(
      ofType(A.updateEvent),
      exhaustMap((action) =>
        this.eventService.updateEvent(action.event).pipe(
          concatLatestFrom(() => this.store.select(selectEmployeesMap)),
          concatLatestFrom(() => this.store.select(selectTeamsMap)),
          map(([[_, employees], teams]) =>
            A.updateEventOk({
              event: {
                ...action.event,
                team: action.event.teamId ? teams[action.event.teamId] : undefined,
                employees: action.event.employeeIds.map((id) => employees[id]),
              },
            }),
          ),
          catchError(() => EMPTY),
        ),
      ),
    ),
  );

  deleteEvent$ = createEffect(() =>
    this.actions.pipe(
      ofType(A.deleteEvent),
      exhaustMap((action) =>
        this.eventService.deleteEvent(action.id).pipe(
          map(() => A.deleteEventOk(action)),
          catchError(() => EMPTY),
        ),
      ),
    ),
  );

  constructor(private actions: Actions, private store: Store<AppState>, private eventService: EventService) {}
}
