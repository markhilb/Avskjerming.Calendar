import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { BaseApiService } from './base-api.service';
import { EventDto } from 'src/app/models/event.model';

@Injectable({
  providedIn: 'root',
})
export class EventService {
  constructor(private api: BaseApiService) {}

  getEvents = (from: Date, to: Date): Observable<EventDto[]> => this.api.get<EventDto[]>('events', { from, to });

  createEvent = (event: EventDto): Observable<number> => this.api.post<number>('events', event);

  updateEvent = (event: EventDto): Observable<boolean> => this.api.put<boolean>('events', {}, event);

  deleteEvent = (id: number): Observable<boolean> => this.api.delete<boolean>('events/' + id);
}
