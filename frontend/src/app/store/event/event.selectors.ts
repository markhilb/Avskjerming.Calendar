import { createSelector } from '@ngrx/store';
import { AppState } from '../app.state';

const eventState = (state: AppState) => state.eventState;

export const _selectEvents = createSelector(eventState, (state) => state.events);

export const selectEvents = createSelector(_selectEvents, (state) =>
  state.map((e) => ({ ...e, start: new Date(e.start), end: new Date(e.end) })),
);

export const selectEventDayStart = createSelector(selectEvents, (state) =>
  Math.min(...state.map((e) => e.start.getHours()), 7),
);
export const selectEventDayEnd = createSelector(selectEvents, (state) =>
  Math.max(...state.map((e) => e.end.getHours()), 22),
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
      id: event.id,
      details: event.details,
      team: event.team,
      employees: event.employees,
    },
  })),
);
