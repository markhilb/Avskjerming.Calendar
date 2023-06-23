import { createSelector } from '@ngrx/store';
import { AppState } from '../app.state';
import { MetaData } from 'src/app/pages/calendar-page/calendar-page.component';
import { CalendarEvent } from 'angular-calendar';

const employeeState = (state: AppState) => state.employeeState;

export const selectEmployees = createSelector(employeeState, (state) => state.employees);

export const selectEmployeesMap = createSelector(selectEmployees, (state) =>
  Object.fromEntries(state?.map((e) => [e.id, e])),
);

export const selectAvailableEmployees = (event: CalendarEvent<MetaData>) =>
  createSelector(selectEmployees, (state) =>
    state.filter((employee) => !event.meta?.employees.some((e) => e.id === employee.id)),
  );
