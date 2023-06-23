import { Employee } from 'src/generated/openapi';

export interface EmployeeState {
  employees: Employee[];
}

export const initialEmployeeState: EmployeeState = {
  employees: [],
};
