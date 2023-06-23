import { Event } from 'src/generated/openapi';

export interface EventState {
  events: Event[];
}

export const initialEventState: EventState = {
  events: [],
};
