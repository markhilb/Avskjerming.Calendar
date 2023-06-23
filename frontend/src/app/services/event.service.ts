import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { environment } from 'src/environments/environment';
import { EventService as BaseEventService, Configuration, CreateEvent, UpdateEvent } from 'src/generated/openapi';

@Injectable({
  providedIn: 'root',
})
export class EventService {
  api: BaseEventService;

  constructor(httpClient: HttpClient) {
    this.api = new BaseEventService(httpClient, environment.baseUrl, new Configuration({ withCredentials: true }));
  }

  getEvents = (start?: Date, end?: Date) => this.api.getEvents(start?.toISOString(), end?.toISOString());

  createEvent = (event: CreateEvent) => this.api.createEvent(event);

  updateEvent = (event: UpdateEvent) => this.api.updateEvent(event);

  deleteEvent = (id: number) => this.api.deleteEvent(id);
}
