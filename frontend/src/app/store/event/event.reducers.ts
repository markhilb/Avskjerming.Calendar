import { createReducer, on } from '@ngrx/store';
import { initialEventState } from './event.state';
import * as A from './event.actions';

export const eventReducer = createReducer(
  initialEventState,
  on(A.getEventsOk, (state, { events }) => ({ ...state, events })),
  on(A.createEventOk, (state, { event }) => ({ ...state, events: [...(state?.events ?? []), event] })),
  on(A.updateEventOk, (state, { event }) => ({
    ...state,
    events: state.events?.map((x) => (x.id === event.id ? event : x)),
  })),
  on(A.deleteEventOk, (state, { id }) => ({ ...state, events: state.events?.filter((x) => x.id !== id) })),
);
