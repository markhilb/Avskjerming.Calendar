import { createAction, props } from '@ngrx/store';
import { CreateEmployee, Employee } from 'src/generated/openapi';

export const getEmployees = createAction('[Employee] Get employees');
export const getEmployeesOk = createAction('[Employee] Get employees Ok', props<{ employees: Employee[] }>());

export const createEmployee = createAction('[Employee] Create employee', props<{ employee: CreateEmployee }>());
export const createEmployeeOk = createAction('[Employee] Create employee Ok', props<{ employee: Employee }>());

export const updateEmployee = createAction('[Employee] Update employee', props<{ employee: Employee }>());
export const updateEmployeeOk = createAction('[Employee] Update employee Ok', props<{ employee: Employee }>());

export const deleteEmployee = createAction('[Employee] Delete employee', props<{ id: number }>());
export const deleteEmployeeOk = createAction('[Employee] Delete employee Ok', props<{ id: number }>());
