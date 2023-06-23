import { Component, ElementRef, HostListener, TemplateRef, ViewChild } from '@angular/core';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
import { Store } from '@ngrx/store';
import { CalendarEvent, CalendarEventTimesChangedEvent, CalendarView } from 'angular-calendar';
import {
  AppState,
  createEvent,
  deleteEvent,
  getEmployees,
  getEvents,
  getTeams,
  selectAvailableEmployees,
  selectCalendarEvents,
  selectEmployees,
  selectEventDayEnd,
  selectEventDayStart,
  selectTeams,
  updateEvent,
} from 'src/app/store';
import { Employee, Event, Team } from 'src/generated/openapi';
import { addDays, addHours, endOfDay, endOfWeek, isSameWeek, startOfDay, startOfWeek } from 'date-fns';
import { interval } from 'rxjs';

export interface MetaData {
  id: number;
  details: string;
  team: Team | null;
  employees: Employee[];
}

const serializeEvent = (event: CalendarEvent<MetaData>): Event => ({
  id: event.meta?.id ?? 0,
  title: event.title,
  details: event.meta?.details ?? '',
  start: event.start.toISOString(),
  end: (event.end ?? event.start).toISOString(),
  team: event.meta?.team ?? null,
  employees: event.meta?.employees ?? [],
});

const _createEvent = (start: Date, end: Date) =>
  ({
    start,
    end,
    title: '',
    color: { primary: 'red', secondary: 'green' },
    resizable: {
      beforeStart: true,
      afterEnd: true,
    },
    draggable: true,
    meta: {
      id: 0,
      details: '',
      team: null,
      employees: [],
    },
  } as CalendarEvent<MetaData>);

@UntilDestroy()
@Component({
  selector: 'app-calendar-page',
  templateUrl: './calendar-page.component.html',
  styleUrls: ['./calendar-page.component.scss'],
})
export class CalendarPageComponent {
  @ViewChild('modalContent', { static: true }) modalContent?: TemplateRef<any>;
  @ViewChild('next', { static: false }) next?: ElementRef;
  @ViewChild('previous', { static: false }) previous?: ElementRef;
  @ViewChild('today', { static: false }) today?: ElementRef;

  events$ = this.store.select(selectCalendarEvents);
  teams$ = this.store.select(selectTeams);
  employees$ = this.store.select(selectEmployees);

  dayStart$ = this.store.select(selectEventDayStart);
  dayEnd$ = this.store.select(selectEventDayEnd);
  excludeDays = [0, 6];

  CalendarView = CalendarView;
  view = window.innerWidth < 500 ? CalendarView.Day : CalendarView.Week;

  startOfWeek = startOfWeek;
  addDays = addDays;

  currentWeek = new Date();

  viewDate = new Date();
  get nextWeekDate() {
    return addDays(this.viewDate, 7);
  }

  modalData?: CalendarEvent<MetaData>;

  @HostListener('document:keydown', ['$event']) keydown(event: KeyboardEvent) {
    if (!this.modal.hasOpenModals()) {
      if (event.key == 'ArrowRight') {
        this.next?.nativeElement.click();
      } else if (event.key === 'ArrowLeft') {
        this.previous?.nativeElement.click();
      } else if (event.key === 'ArrowUp' || event.key === 'ArrowDown') {
        this.today?.nativeElement.click();
      } else if (event.key === 'Tab') {
        this.createEvent({ date: new Date() });
        event.preventDefault();
      }
    } else {
      if (event.key === 'Escape') {
        this.modal.dismissAll();
      }
    }
  }

  constructor(private store: Store<AppState>, private modal: NgbModal) {
    this.fetchEvents();
    interval(1000 * 60)
      .pipe(untilDestroyed(this))
      .subscribe(() => this.fetchEvents());

    store.dispatch(getTeams());
    store.dispatch(getEmployees());

    interval(1000 * 60 * 60)
      .pipe(untilDestroyed(this))
      .subscribe(() => {
        if (!isSameWeek(new Date(), this.currentWeek)) {
          this.currentWeek = new Date();
          this.viewDate = this.nextWeekDate;
          this.fetchEvents();
        }
      });
  }

  fetchEvents() {
    this.store.dispatch(
      getEvents(
        this.view === CalendarView.Day
          ? { start: startOfDay(this.viewDate), end: endOfDay(this.viewDate) }
          : this.view === CalendarView.Week
          ? { start: startOfWeek(this.viewDate), end: endOfWeek(this.viewDate) }
          : { start: startOfWeek(this.viewDate), end: endOfWeek(this.nextWeekDate) },
      ),
    );
  }

  availableEmployees(event: CalendarEvent<MetaData>) {
    return this.store.select(selectAvailableEmployees(event));
  }

  openModal(event: CalendarEvent<MetaData>): void {
    this.modalData = { ...event, meta: event.meta ? { ...event.meta } : undefined };
    this.modal.open(this.modalContent, { size: 'lg', centered: true });
  }

  createEvent({ date }: { date: Date }) {
    const event = _createEvent(date, addHours(date, 1));
    this.openModal(event);
    setTimeout(() => document.getElementById('title')?.focus(), 0);
  }

  confirmCreateEvent() {
    if (this.modalData) {
      const event = {
        ...serializeEvent(this.modalData),
        teamId: this.modalData.meta?.team?.id,
        employeeIds: this.modalData.meta?.employees.map((e) => e.id) ?? [],
      };
      this.store.dispatch(createEvent({ event }));
      this.modal.dismissAll();
    }
  }

  updateEvent() {
    if (this.modalData) {
      const event = {
        ...serializeEvent(this.modalData),
        teamId: this.modalData.meta?.team?.id,
        employeeIds: this.modalData.meta?.employees.map((e) => e.id) ?? [],
      };
      this.store.dispatch(updateEvent({ event }));
      this.modal.dismissAll();
    }
  }

  eventTimesChanged({ event, newStart, newEnd }: CalendarEventTimesChangedEvent<MetaData>) {
    const updatedEvent = {
      ...serializeEvent(event),
      start: newStart.toISOString(),
      end: (newEnd ?? newStart).toISOString(),
      teamId: event.meta?.team?.id,
      employeeIds: event.meta?.employees.map((e) => e.id) ?? [],
    };
    this.store.dispatch(updateEvent({ event: updatedEvent }));
  }

  deleteEvent(event: CalendarEvent<MetaData>) {
    if (event.meta) {
      this.store.dispatch(deleteEvent({ id: event.meta.id }));
      this.modal.dismissAll();
    }
  }

  changeTeam(team: Team) {
    if (this.modalData?.meta) {
      this.modalData.meta.team = team;
      this.modalData.color = { primary: team.primaryColor, secondary: team.secondaryColor };
    }
  }

  addEmployee(employee: Employee) {
    if (this.modalData?.meta) {
      this.modalData.meta.employees = [...this.modalData.meta.employees, employee];
    }
  }

  removeEmployee(employee: Employee) {
    if (this.modalData?.meta) {
      this.modalData.meta.employees = this.modalData.meta.employees.filter((e) => e.id !== employee.id);
    }
  }
}
