import { createAction, props } from '@ngrx/store';
import { CreateEvent, Event, UpdateEvent } from 'src/generated/openapi';

export const getEvents = createAction('[Event] Get events', props<{ start?: Date; end?: Date }>());
export const getEventsOk = createAction('[Event] Get events Ok', props<{ events: Event[] }>());

export const createEvent = createAction('[Event] Create event', props<{ event: CreateEvent }>());
export const createEventOk = createAction('[Event] Create event Ok', props<{ event: Event }>());

export const updateEvent = createAction('[Event] Update event', props<{ event: UpdateEvent }>());
export const updateEventOk = createAction('[Event] Update event Ok', props<{ event: Event }>());

export const deleteEvent = createAction('[Event] Delete event', props<{ id: number }>());
export const deleteEventOk = createAction('[Event] Delete event Ok', props<{ id: number }>());
