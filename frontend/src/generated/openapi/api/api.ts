export * from './auth.service';
import { AuthService } from './auth.service';
export * from './employee.service';
import { EmployeeService } from './employee.service';
export * from './event.service';
import { EventService } from './event.service';
export * from './team.service';
import { TeamService } from './team.service';
export const APIS = [AuthService, EmployeeService, EventService, TeamService];
