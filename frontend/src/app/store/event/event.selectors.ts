import { createSelector } from '@ngrx/store';
import { selectAppState } from '../app.reducers';

export const selectEventState = createSelector(selectAppState, (state) => state.eventState);

export const selectEvents = createSelector(selectEventState, (state) => state.events);

export const selectEventDayStart = createSelector(selectEvents, (state) =>
  Math.min(...state.map((e) => e.start.getHours())),
);
export const selectEventDayEnd = createSelector(selectEvents, (state) =>
  Math.max(...state.map((e) => e.start.getHours())),
);

export const selectCalendarEvents = createSelector(selectEvents, (state) =>
  state.map((event) => ({
    start: event.start,
    end: event.end,
    title: event.title,
    color: event.team
      ? { primary: event.team.primaryColor, secondary: event.team.secondaryColor }
      : { primary: '#ffffff', secondary: '#aaaaaa' },
    resizable: {
      beforeStart: true,
      afterEnd: true,
    },
    draggable: true,
    meta: {
      details: event.details,
      team: event.team,
      employees: event.employees,
      id: event.id,
    },
  })),
);
