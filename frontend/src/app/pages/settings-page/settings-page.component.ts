import { Component, TemplateRef, ViewChild } from '@angular/core';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { Store } from '@ngrx/store';
import {
  AppState,
  createEmployee,
  createTeam,
  deleteEmployee,
  deleteTeam,
  getEmployees,
  getTeams,
  selectEmployees,
  selectTeams,
  updateEmployee,
  updateTeam,
} from 'src/app/store';
import { environment } from 'src/environments/environment';
import { Employee, Team } from 'src/generated/openapi';

@Component({
  selector: 'app-settings-page',
  templateUrl: './settings-page.component.html',
  styleUrls: ['./settings-page.component.scss'],
})
export class SettingsPageComponent {
  @ViewChild('addTeamModal', { static: true }) addTeamModal?: TemplateRef<any>;
  @ViewChild('addEmployeeModal', { static: true }) addEmployeeModal?: TemplateRef<any>;

  teams$ = this.store.select(selectTeams);
  employees$ = this.store.select(selectEmployees);

  modalTeam?: Team;
  modalEmployee?: Employee;

  auth = environment.auth;

  constructor(private modal: NgbModal, private store: Store<AppState>) {
    store.dispatch(getTeams());
    store.dispatch(getEmployees());
  }

  addTeam() {
    this.modalTeam = { id: 0, name: '', primaryColor: '#ffffff', secondaryColor: '#bbbbbb', disabled: false };
    this.modal.open(this.addTeamModal, { size: 'lg', centered: true });
    setTimeout(() => document.getElementById('teamName')?.focus(), 0);
  }

  confirmAddTeam() {
    if (this.modalTeam) {
      this.store.dispatch(createTeam({ team: this.modalTeam }));
      this.modal.dismissAll();
    }
  }

  updateTeam(team: Team) {
    this.modalTeam = { ...team };
    this.modal.open(this.addTeamModal, { size: 'lg', centered: true });
    setTimeout(() => document.getElementById('teamName')?.focus(), 0);
  }

  confirmUpdateTeam() {
    if (this.modalTeam) {
      this.store.dispatch(updateTeam({ team: this.modalTeam }));
      this.modal.dismissAll();
    }
  }

  deleteTeam(id: number) {
    this.store.dispatch(deleteTeam({ id }));
  }

  addEmployee() {
    this.modalEmployee = { id: 0, name: '', color: '#ffffff', disabled: false };
    this.modal.open(this.addEmployeeModal, { size: 'lg', centered: true });
    setTimeout(() => document.getElementById('employeeName')?.focus(), 0);
  }

  confirmAddEmployee() {
    if (this.modalEmployee) {
      this.store.dispatch(createEmployee({ employee: this.modalEmployee }));
      this.modal.dismissAll();
    }
  }

  updateEmployee(employee: Employee) {
    this.modalEmployee = { ...employee };
    this.modal.open(this.addEmployeeModal, { size: 'lg', centered: true });
    setTimeout(() => document.getElementById('employeeName')?.focus(), 0);
  }

  confirmUpdateEmployee() {
    if (this.modalEmployee) {
      this.store.dispatch(updateEmployee({ employee: this.modalEmployee }));
      this.modal.dismissAll();
    }
  }

  deleteEmployee(id: number) {
    this.store.dispatch(deleteEmployee({ id }));
  }
}
