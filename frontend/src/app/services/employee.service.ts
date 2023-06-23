import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { environment } from 'src/environments/environment';
import { EmployeeService as BaseEmployeeService, Configuration, CreateEmployee, Employee } from 'src/generated/openapi';

@Injectable({
  providedIn: 'root',
})
export class EmployeeService {
  api: BaseEmployeeService;

  constructor(httpClient: HttpClient) {
    this.api = new BaseEmployeeService(httpClient, environment.baseUrl, new Configuration({ withCredentials: true }));
  }

  getEmployees = () => this.api.getEmployees();

  createEmployee = (employee: CreateEmployee) => this.api.createEmployee(employee);

  updateEmployee = (employee: Employee) => this.api.updateEmployee(employee);

  deleteEmployee = (id: number) => this.api.deleteEmployee(id);
}
